pub fn day1_2() {
  let input_modules: [u32; 100] = [
    140170, 75120, 75645, 134664, 124948, 137630, 146662, 116881, 120030, 94332, 50473, 59361,
    128237, 84894, 51368, 128802, 57275, 129235, 113481, 66378, 55842, 90548, 107696, 53603,
    130458, 80306, 120820, 131313, 100303, 59224, 123369, 140584, 60642, 68184, 103101, 82278,
    51968, 51048, 98139, 60498, 127082, 71197, 109478, 71286, 84840, 141305, 51800, 72352, 93147,
    73549, 122739, 62363, 58453, 59000, 63564, 63424, 51053, 120826, 123337, 130824, 59053, 77983,
    68977, 67126, 96051, 53024, 145647, 139343, 113236, 59396, 146174, 148622, 83384, 86938,
    100673, 80757, 107675, 147417, 124538, 136463, 104609, 149559, 136037, 54997, 139674, 101638,
    65739, 70029, 143847, 122035, 66256, 78087, 105045, 108867, 99630, 127173, 139021, 139759,
    134171, 104869,
  ];

  let mut required_fuel = 0;

  for module in input_modules.iter() {
    required_fuel += calculate_required_fuel(module);
  }

  println!("The required fuel is {}", required_fuel);
}

fn calculate_required_fuel(mass: &u32) -> u32 {
  let required_fuel = match (mass / 3).checked_sub(2) {
    None => 0,
    Some(i) => i,
  };
  if required_fuel == 0 {
    return 0;
  } else {
    return required_fuel + calculate_required_fuel(&required_fuel);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn calculate_required_fuel_returns_correct() {
    assert_eq!(calculate_required_fuel(&3), 0);
    assert_eq!(calculate_required_fuel(&6), 0);
    assert_eq!(calculate_required_fuel(&9), 1);
    assert_eq!(calculate_required_fuel(&12), 2);
    assert_eq!(calculate_required_fuel(&15), 3);
    assert_eq!(calculate_required_fuel(&27), 7);
    assert_eq!(calculate_required_fuel(&36), 11);
    assert_eq!(calculate_required_fuel(&51), 18);
  }
}
