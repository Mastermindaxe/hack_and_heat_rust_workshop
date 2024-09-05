#[derive(Default, Debug)]
pub struct HackAndHeat {
    room_temperature: i8,
    attendees: Vec<Attendee>,
}

// TODO: What does this macro do?
#[derive(Default, Debug)]
pub struct Attendee {
    name: String,
    age: u16,
}

fn main() {
    let mike = Attendee {
        name: "Mike".to_string(),
        age: 25,
    };
    let max = Attendee::default();

    let hack_and_heat = HackAndHeat {
        room_temperature: 25,
        attendees: vec![mike, max],
    };

    let default_hack_and_heat = HackAndHeat {
        room_temperature: 30,
        ..Default::default()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_01() {
        assert_eq!("24", "24");
    }
}
