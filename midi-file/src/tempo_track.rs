use midly::{MetaMessage, TrackEvent, TrackEventKind};
use std::{collections::HashMap, sync::Arc, time::Duration};

#[derive(Debug, Clone)]
pub struct TempoEvent {
    pub absolute_pulses: u64,
    pub timestamp: Duration,
    /// Tempo in microseconds per quarter note.
    pub tempo: u32,
}

#[derive(Debug, Clone)]
pub struct TempoTrack {
    pulses_per_quarter_note: u16,
    events: Arc<[TempoEvent]>,
}

impl TempoTrack {
    pub fn build(track_events: &[Vec<TrackEvent>], pulses_per_quarter_note: u16) -> TempoTrack {
        // This map will help us get rid of duplicate events if
        // the tempo is specified in every track (as is common).
        let mut tempo_events: HashMap<u64, TempoEvent> = HashMap::new();

        for events in track_events.iter() {
            let mut pulses: u64 = 0;
            for event in events.iter() {
                pulses += event.delta.as_int() as u64;

                if let TrackEventKind::Meta(MetaMessage::Tempo(t)) = &event.kind {
                    tempo_events.insert(
                        pulses,
                        TempoEvent {
                            absolute_pulses: pulses,
                            timestamp: Duration::ZERO,
                            tempo: t.as_int(),
                        },
                    );
                };
            }
        }

        let mut tempo_events: Vec<_> = tempo_events.into_values().collect();
        tempo_events.sort_by_key(|e| e.absolute_pulses);

        let mut previous_absolute_pulses = 0_u64;
        let mut running_tempo = 500_000;
        let mut res = Duration::ZERO;

        for tempo_event in tempo_events.iter_mut() {
            let tempo_event_pulses = tempo_event.absolute_pulses;

            let relative_pulses = tempo_event_pulses - previous_absolute_pulses;

            res += pulse_to_duration(relative_pulses, running_tempo, pulses_per_quarter_note);

            tempo_event.timestamp = res;

            running_tempo = tempo_event.tempo;
            previous_absolute_pulses = tempo_event_pulses;
        }

        TempoTrack {
            pulses_per_quarter_note,
            events: tempo_events.into(),
        }
    }

    pub fn tempo_event_for_pulses(&self, pulses: u64) -> Option<&TempoEvent> {
        let res = self
            .events
            .binary_search_by_key(&pulses, |e| e.absolute_pulses);

        let id = match res {
            Ok(id) => Some(id),
            Err(id) => id.checked_sub(1),
        };

        id.and_then(|id| self.events.get(id))
    }

    pub fn pulses_to_duration(&self, event_pulses: u64) -> Duration {
        let tempo_event = self.tempo_event_for_pulses(event_pulses);

        let (res, previous_absolute_pulses, tempo) = if let Some(event) = tempo_event {
            (event.timestamp, event.absolute_pulses, event.tempo)
        } else {
            // 120 BPM
            let default_tempo = 500_000;
            (Duration::ZERO, 0, default_tempo)
        };

        let delta_pulses = event_pulses - previous_absolute_pulses;
        res + pulse_to_duration(delta_pulses, tempo, self.pulses_per_quarter_note)
    }
}

fn pulse_to_duration(pulses: u64, tempo: u32, pulses_per_quarter_note: u16) -> Duration {
    let u_time = pulses as f64 / pulses_per_quarter_note as f64;
    // We floor only because Synthesia floors,
    // so if we want to test for timing regresions we have to do the same
    let time = (u_time * tempo as f64).floor() as u64;
    Duration::from_micros(time)
}
