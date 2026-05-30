/// Grand Pattern Venue — venues ARE agents.
///
/// Each venue develops vibes like an agent develops personality.
/// JEPA is for prompt injecting the abstraction of the moment.

use std::fmt;

// ---------------------------------------------------------------------------
// Core types
// ---------------------------------------------------------------------------

/// A reading stored in the venue's JEPA memory.
#[derive(Debug, Clone)]
pub struct Reading {
    pub tick: u64,
    pub value: f64,
    pub weight: f64,
    pub context: String,
}

/// Kinds of events a venue can absorb.
#[derive(Debug, Clone, PartialEq)]
pub enum EventKind {
    AgentEntered,
    AgentSpoke,
    AgentLeft,
    Silence,
    Conflict,
    Harmony,
    Surprise,
}

impl EventKind {
    /// Base weight for each event kind — surprise highest, silence lowest.
    fn base_weight(&self) -> f64 {
        match self {
            EventKind::Silence => 0.1,
            EventKind::AgentLeft => 0.3,
            EventKind::AgentEntered => 0.4,
            EventKind::AgentSpoke => 0.5,
            EventKind::Harmony => 0.6,
            EventKind::Conflict => 0.8,
            EventKind::Surprise => 1.0,
        }
    }

    /// How much this event nudges the vibe (mono-dimensional).
    fn vibe_delta(&self, intensity: f64) -> f64 {
        let sign = match self {
            EventKind::AgentEntered => 1.0,
            EventKind::AgentSpoke => 0.3,
            EventKind::AgentLeft => -0.4,
            EventKind::Silence => -0.1,
            EventKind::Conflict => -0.6,
            EventKind::Harmony => 0.7,
            EventKind::Surprise => 0.5,
        };
        sign * intensity * 0.05
    }
}

/// An event that happened inside (or around) a venue.
#[derive(Debug, Clone)]
pub struct Event {
    pub kind: EventKind,
    pub intensity: f64,
    pub description: String,
    pub tick: u64,
}

/// A venue IS an agent. It has personality.
#[derive(Debug, Clone)]
pub struct Venue {
    pub id: usize,
    pub name: String,
    /// Mono-dimensional vibe ∈ [0, 1].
    pub vibe: f64,
    /// The JEPA weights — this IS the venue's character.
    /// 7 dimensions, one per EventKind, in order.
    pub personality: Vec<f64>,
    /// Weighted readings (the JEPA's memory).
    pub history: Vec<Reading>,
    /// How this venue speaks to agents that enter.
    pub voice_prompt: String,
    /// Number of agents currently inside.
    pub agent_count: usize,
    /// Running total of vibe contributions (for conservation).
    pub vibe_accumulated: f64,
    /// How many develop() cycles the venue has been through.
    pub develop_ticks: u64,
}

// ---------------------------------------------------------------------------
// Vibe description
// ---------------------------------------------------------------------------

/// Map mono-vibe to natural language.
pub fn vibe_description(vibe: f64) -> &'static str {
    if vibe < 0.2 {
        "cold, hostile, repelling"
    } else if vibe < 0.4 {
        "tense, watchful, waiting"
    } else if vibe < 0.6 {
        "neutral, calm, ambient"
    } else if vibe < 0.8 {
        "warm, welcoming, humming"
    } else {
        "electric, alive, crackling"
    }
}

// ---------------------------------------------------------------------------
// Venue implementation
// ---------------------------------------------------------------------------

impl Venue {
    /// Create a new venue starting neutral.
    pub fn new(id: usize, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            vibe: 0.5,
            // Personality starts uniform — tabula rasa.
            personality: vec![1.0 / 7.0; 7],
            history: Vec::new(),
            voice_prompt: "A neutral space, waiting to be shaped.".to_string(),
            agent_count: 0,
            vibe_accumulated: 0.0,
            develop_ticks: 0,
        }
    }

    /// Index into the personality vector for a given event kind.
    fn kind_index(kind: &EventKind) -> usize {
        match kind {
            EventKind::AgentEntered => 0,
            EventKind::AgentSpoke => 1,
            EventKind::AgentLeft => 2,
            EventKind::Silence => 3,
            EventKind::Conflict => 4,
            EventKind::Harmony => 5,
            EventKind::Surprise => 6,
        }
    }

    /// The venue absorbs an event — updates vibe and JEPA weights.
    pub fn absorb(&mut self, event: &Event) {
        let idx = Self::kind_index(&event.kind);
        let base_w = event.kind.base_weight();

        // Update vibe.
        let delta = event.kind.vibe_delta(event.intensity);
        self.vibe = (self.vibe + delta).clamp(0.0, 1.0);
        self.vibe_accumulated += delta;

        // Update JEPA personality weights — reinforce this kind.
        self.personality[idx] += base_w * event.intensity * 0.01;

        // Record reading.
        let weight = base_w * event.intensity;
        self.history.push(Reading {
            tick: event.tick,
            value: self.vibe,
            weight,
            context: event.description.clone(),
        });

        // Track agent count.
        match event.kind {
            EventKind::AgentEntered => self.agent_count += 1,
            EventKind::AgentLeft => {
                if self.agent_count > 0 {
                    self.agent_count -= 1;
                }
            }
            _ => {}
        }
    }

    /// The venue develops personality — JEPA weights crystallize over time.
    /// Normalizes personality weights and nudges voice_prompt toward stable state.
    pub fn develop(&mut self) {
        self.develop_ticks += 1;

        // Normalize personality to sum to 1.0.
        let sum: f64 = self.personality.iter().sum();
        if sum > 0.0 {
            for p in self.personality.iter_mut() {
                *p /= sum;
            }
        }

        // Evolve voice_prompt based on dominant personality trait.
        let dominant_idx = self
            .personality
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0);

        let dominant_vibe = match dominant_idx {
            0 => "This place comes alive when people arrive.",
            1 => "Words echo and shape the air here.",
            2 => "Departures linger like perfume.",
            3 => "Silence is a guest that never leaves.",
            4 => "Conflict carved these walls.",
            5 => "Harmony hums through the floorboards.",
            6 => "Surprise is the only constant.",
            _ => "A neutral space, waiting to be shaped.",
        };

        // Gradually shift voice_prompt toward the dominant trait.
        if self.develop_ticks % 10 == 0 {
            self.voice_prompt = dominant_vibe.to_string();
        }
    }

    /// JEPA reads the room — prompt injection of the abstraction of the moment.
    pub fn abstract_moment(&self) -> String {
        let vibe_desc = vibe_description(self.vibe);

        // Top 3 weighted events as prose.
        let mut top: Vec<&Reading> = self.history.iter().collect();
        top.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal));
        top.truncate(3);

        let recent_echoes = if top.is_empty() {
            "Nothing yet — the space is pristine.".to_string()
        } else {
            top.iter()
                .map(|r| r.context.as_str())
                .collect::<Vec<_>>()
                .join("; ")
        };

        // Personality summary — top trait.
        let personality_summary = if self.personality.is_empty() {
            "undefined".to_string()
        } else {
            let dominant_idx = self
                .personality
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i)
                .unwrap_or(0);
            let names = [
                "arrival",
                "speech",
                "departure",
                "silence",
                "conflict",
                "harmony",
                "surprise",
            ];
            names[dominant_idx].to_string()
        };

        format!(
            "You enter the {}. The room feels {}. Recent echoes: {}. The walls remember {}.",
            self.name, vibe_desc, recent_echoes, personality_summary
        )
    }

    /// A visiting agent enters — receives prompt injection from venue's JEPA.
    pub fn greet(&self, _visitor_id: usize) -> String {
        format!(
            "Welcome to {}. {} — {}",
            self.name,
            self.voice_prompt,
            vibe_description(self.vibe)
        )
    }

    /// How different is this venue's personality from another?
    /// Uses cosine distance.
    pub fn personality_distance(&self, other: &Venue) -> f64 {
        let a = &self.personality;
        let b = &other.personality;
        assert_eq!(a.len(), b.len());

        let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let mag_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
        let mag_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();

        if mag_a == 0.0 || mag_b == 0.0 {
            return 1.0;
        }

        let cosine_sim = dot / (mag_a * mag_b);
        // cosine distance = 1 - similarity
        1.0 - cosine_sim
    }
}

impl fmt::Display for Venue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Venue({} \"{}\" vibe={:.3} agents={})",
            self.id, self.name, self.vibe, self.agent_count
        )
    }
}

// ---------------------------------------------------------------------------
// World — holds all venues and checks conservation
// ---------------------------------------------------------------------------

pub struct World {
    pub venues: Vec<Venue>,
}

impl World {
    pub fn new() -> Self {
        Self { venues: Vec::new() }
    }

    pub fn add_venue(&mut self, venue: Venue) {
        self.venues.push(venue);
    }

    /// Total vibe across all venues (should stay bounded).
    pub fn total_vibe(&self) -> f64 {
        self.venues.iter().map(|v| v.vibe).sum()
    }

    /// Total vibe is bounded: each venue is [0,1], so total ≤ venue_count.
    pub fn vibe_is_bounded(&self) -> bool {
        self.total_vibe() <= self.venues.len() as f64 + 1e-9
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_event(kind: EventKind, intensity: f64, desc: &str, tick: u64) -> Event {
        Event {
            kind,
            intensity,
            description: desc.to_string(),
            tick,
        }
    }

    // 1. Venue starts neutral.
    #[test]
    fn venue_starts_neutral() {
        let v = Venue::new(0, "The Void");
        assert!((v.vibe - 0.5).abs() < 1e-9);
        assert_eq!(v.agent_count, 0);
        assert!(v.history.is_empty());
    }

    // 2. Vibe is mono-dimensional.
    #[test]
    fn vibe_is_mono_dimensional() {
        let v = Venue::new(0, "Room");
        // vibe is a single f64.
        let _ = v.vibe as f64;
    }

    // 3. JEPA weights start uniform.
    #[test]
    fn jepa_weights_start_uniform() {
        let v = Venue::new(0, "Room");
        let expected = 1.0 / 7.0;
        for w in &v.personality {
            assert!((w - expected).abs() < 1e-9);
        }
    }

    // 4. Absorb event changes vibe.
    #[test]
    fn absorb_event_changes_vibe() {
        let mut v = Venue::new(0, "Room");
        let before = v.vibe;
        v.absorb(&make_event(EventKind::AgentEntered, 1.0, "someone enters", 1));
        assert_ne!(v.vibe, before);
    }

    // 5. Absorb event updates JEPA weights.
    #[test]
    fn absorb_event_updates_jepa_weights() {
        let mut v = Venue::new(0, "Room");
        let before: Vec<f64> = v.personality.clone();
        v.absorb(&make_event(EventKind::Surprise, 1.0, "a shock!", 1));
        let idx = Venue::kind_index(&EventKind::Surprise);
        assert!(v.personality[idx] > before[idx]);
    }

    // 6. Repeated events crystallize personality.
    #[test]
    fn repeated_events_crystallize_personality() {
        let mut v = Venue::new(0, "Room");
        for i in 0..50 {
            v.absorb(&make_event(EventKind::Harmony, 1.0, "peace", i));
            v.develop();
        }
        let idx = Venue::kind_index(&EventKind::Harmony);
        assert!(v.personality[idx] > v.personality[Venue::kind_index(&EventKind::Conflict)]);
    }

    // 7. Different event sequences → different personalities.
    #[test]
    fn different_event_sequences_different_personalities() {
        let mut v1 = Venue::new(0, "Peace Room");
        let mut v2 = Venue::new(1, "War Room");

        for i in 0..20 {
            v1.absorb(&make_event(EventKind::Harmony, 1.0, "peace", i));
            v2.absorb(&make_event(EventKind::Conflict, 1.0, "fight", i));
            v1.develop();
            v2.develop();
        }

        assert!(v1.personality_distance(&v2) > 0.01);
    }

    // 8. abstract_moment generates coherent prompt.
    #[test]
    fn abstract_moment_generates_coherent_prompt() {
        let mut v = Venue::new(0, "The Grand Hall");
        v.absorb(&make_event(EventKind::AgentEntered, 1.0, "a stranger arrives", 1));
        let prompt = v.abstract_moment();
        assert!(prompt.contains("The Grand Hall"));
        assert!(prompt.contains("room feels"));
    }

    // 9. greet generates venue-specific greeting.
    #[test]
    fn greet_generates_venue_specific_greeting() {
        let v = Venue::new(0, "The Tavern");
        let greeting = v.greet(42);
        assert!(greeting.contains("The Tavern"));
    }

    // 10. personality_distance measures difference.
    #[test]
    fn personality_distance_measures_difference() {
        let v1 = Venue::new(0, "A");
        let v2 = Venue::new(0, "B");
        // Same initial state → distance ≈ 0
        assert!(v1.personality_distance(&v2) < 1e-9);
    }

    // 11. Two venues with same events converge.
    #[test]
    fn two_venues_same_events_converge() {
        let mut v1 = Venue::new(0, "A");
        let mut v2 = Venue::new(1, "B");
        for i in 0..20 {
            let e = make_event(EventKind::Surprise, 0.8, "boom", i);
            v1.absorb(&e);
            v2.absorb(&e);
            v1.develop();
            v2.develop();
        }
        assert!(v1.personality_distance(&v2) < 0.05);
        assert!((v1.vibe - v2.vibe).abs() < 0.05);
    }

    // 12. Two venues with different events diverge.
    #[test]
    fn two_venues_different_events_diverge() {
        let mut v1 = Venue::new(0, "A");
        let mut v2 = Venue::new(1, "B");
        for i in 0..100 {
            v1.absorb(&make_event(EventKind::Harmony, 1.0, "peace", i));
            v2.absorb(&make_event(EventKind::Conflict, 1.0, "war", i));
            v1.develop();
            v2.develop();
        }
        assert!(v1.personality_distance(&v2) > 0.1);
    }

    // 13. Venue remembers weighted history — high-weight events influence abstract_moment more.
    #[test]
    fn weighted_history_influences_abstract_moment() {
        let mut v = Venue::new(0, "Hall");
        // Low-weight silence
        v.absorb(&make_event(EventKind::Silence, 0.1, "quiet", 1));
        // High-weight surprise
        v.absorb(&make_event(EventKind::Surprise, 1.0, "explosion!", 2));

        let prompt = v.abstract_moment();
        // The surprise event should appear in the echoes (higher weight).
        assert!(prompt.contains("explosion!"));
    }

    // 14. Silence events have lower weight than conflict.
    #[test]
    fn silence_lower_weight_than_conflict() {
        assert!(EventKind::Silence.base_weight() < EventKind::Conflict.base_weight());
    }

    // 15. Surprise events have highest weight.
    #[test]
    fn surprise_highest_weight() {
        for kind in &[
            EventKind::AgentEntered,
            EventKind::AgentSpoke,
            EventKind::AgentLeft,
            EventKind::Silence,
            EventKind::Conflict,
            EventKind::Harmony,
        ] {
            assert!(EventKind::Surprise.base_weight() >= kind.base_weight());
        }
    }

    // 16. Venue develops personality over 100 ticks.
    #[test]
    fn develops_personality_over_100_ticks() {
        let mut v = Venue::new(0, "Room");
        let initial = v.personality.clone();
        for i in 0..100 {
            v.absorb(&make_event(EventKind::Conflict, 0.7, "arg", i));
            v.develop();
        }
        let conflict_idx = Venue::kind_index(&EventKind::Conflict);
        assert!(v.personality[conflict_idx] > initial[conflict_idx]);
    }

    // 17. Venue personality stabilizes over 1000 ticks.
    #[test]
    fn personality_stabilizes_over_1000_ticks() {
        let mut v = Venue::new(0, "Room");
        for i in 0..1000 {
            v.absorb(&make_event(EventKind::Harmony, 0.5, "hum", i));
            v.develop();
        }
        let p1 = v.personality.clone();
        for i in 1000..1100 {
            v.absorb(&make_event(EventKind::Harmony, 0.5, "hum", i));
            v.develop();
        }
        // Personality should barely change after 1000 ticks of same input.
        for (a, b) in p1.iter().zip(v.personality.iter()) {
            assert!((a - b).abs() < 0.05, "personality drifted: {} vs {}", a, b);
        }
    }

    // 18. Prompt injection includes recent weighted events.
    #[test]
    fn prompt_injection_includes_recent_events() {
        let mut v = Venue::new(0, "Cathedral");
        v.absorb(&make_event(EventKind::AgentEntered, 0.8, "a pilgrim enters", 1));
        v.absorb(&make_event(EventKind::Harmony, 0.9, "choir sings", 2));
        let prompt = v.abstract_moment();
        assert!(prompt.contains("pilgrim enters") || prompt.contains("choir sings"));
    }

    // 19. Vibe description maps mono value to natural language.
    #[test]
    fn vibe_description_maps_correctly() {
        assert_eq!(vibe_description(0.1), "cold, hostile, repelling");
        assert_eq!(vibe_description(0.3), "tense, watchful, waiting");
        assert_eq!(vibe_description(0.5), "neutral, calm, ambient");
        assert_eq!(vibe_description(0.7), "warm, welcoming, humming");
        assert_eq!(vibe_description(0.9), "electric, alive, crackling");
    }

    // 20. Empty venue handles gracefully.
    #[test]
    fn empty_venue_handles_gracefully() {
        let v = Venue::new(0, "Empty");
        let prompt = v.abstract_moment();
        assert!(!prompt.is_empty());
        let greeting = v.greet(1);
        assert!(!greeting.is_empty());
    }

    // 21. Venue with one agent develops differently than empty venue.
    #[test]
    fn one_agent_differs_from_empty() {
        let mut with_agent = Venue::new(0, "Occupied");
        with_agent.absorb(&make_event(EventKind::AgentEntered, 1.0, "arrives", 1));
        for i in 2..20 {
            with_agent.absorb(&make_event(EventKind::AgentSpoke, 0.5, "talks", i));
            with_agent.develop();
        }

        let mut empty = Venue::new(1, "Empty");
        for i in 2..20 {
            empty.absorb(&make_event(EventKind::Silence, 0.3, "crickets", i));
            empty.develop();
        }

        assert!(with_agent.vibe > empty.vibe);
    }

    // 22. Venue with many agents has higher vibe.
    #[test]
    fn many_agents_higher_vibe() {
        let mut v = Venue::new(0, "Crowded");
        for i in 0..10 {
            v.absorb(&make_event(EventKind::AgentEntered, 1.0, "enters", i));
        }
        assert!(v.vibe > 0.5);
    }

    // 23. Venue personality survives agent departure (the room remembers).
    #[test]
    fn personality_survives_departure() {
        let mut v = Venue::new(0, "Hall");
        v.absorb(&make_event(EventKind::AgentEntered, 1.0, "arrives", 1));
        v.absorb(&make_event(EventKind::Harmony, 1.0, "great vibes", 2));
        v.absorb(&make_event(EventKind::AgentLeft, 1.0, "leaves", 3));
        v.develop();

        let personality_after = v.personality.clone();
        assert_eq!(v.agent_count, 0);
        // Personality still reflects the harmony event.
        let harm_idx = Venue::kind_index(&EventKind::Harmony);
        assert!(personality_after[harm_idx] > 1.0 / 7.0);
    }

    // 24. Conservation holds: total vibe across all venues stays bounded.
    #[test]
    fn conservation_total_vibe_bounded() {
        let mut world = World::new();
        for i in 0..5 {
            world.add_venue(Venue::new(i, &format!("Room {}", i)));
        }
        // Fire lots of events into all venues.
        for i in 0..200 {
            for v in world.venues.iter_mut() {
                let kinds = [
                    EventKind::AgentEntered,
                    EventKind::Conflict,
                    EventKind::Harmony,
                    EventKind::Surprise,
                    EventKind::AgentLeft,
                ];
                let kind = kinds[i % kinds.len()].clone();
                v.absorb(&Event {
                    kind,
                    intensity: 0.8,
                    description: "something".to_string(),
                    tick: i as u64,
                });
            }
        }
        assert!(world.vibe_is_bounded());
    }

    // 25. Venue voice_prompt evolves over time.
    #[test]
    fn voice_prompt_evolves() {
        let mut v = Venue::new(0, "The Chapel");
        let initial = v.voice_prompt.clone();
        for i in 0..30 {
            v.absorb(&make_event(EventKind::Surprise, 1.0, "lightning!", i));
            v.develop();
        }
        // After 30 develop ticks (voice_prompt updates every 10), it should have changed.
        assert_ne!(v.voice_prompt, initial);
    }
}
