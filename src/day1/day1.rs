fn fuel_for_mass(mass: u32) -> u32 {
    if mass / 3 < 2 {
        return 0;
    }
    mass / 3 - 2
}

fn fuel_for_module_and_fuel(mass: u32) -> u32 {
    if mass == 0 {
        return 0;
    }

    fuel_for_mass(mass) + fuel_for_module_and_fuel(fuel_for_mass(mass))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_for_mass() {
        assert_eq!(2, fuel_for_mass(12));
        assert_eq!(2, fuel_for_mass(14));
        assert_eq!(654, fuel_for_mass(1969));
        assert_eq!(33583, fuel_for_mass(100756));
        assert_eq!(0, fuel_for_mass(5));
    }

    const INPUT: &[u32] = &[
        102777, 107296, 131207, 116508, 99009, 120098, 83121, 87846, 126604, 79906, 63668, 143932,
        51829, 106383, 121354, 138556, 123426, 111544, 84395, 147066, 61897, 133724, 75867, 106697,
        67782, 86191, 50666, 138928, 118740, 136863, 123108, 85168, 138487, 115656, 104811, 114986,
        147241, 73860, 99186, 134657, 98379, 59914, 144863, 119851, 82549, 93564, 79437, 70761,
        134303, 108109, 116208, 80702, 111018, 131996, 119367, 74305, 65905, 116871, 102184,
        101880, 100453, 111281, 103134, 129529, 133885, 76153, 56890, 86262, 52804, 139907, 131360,
        80009, 121015, 74438, 54470, 73386, 112961, 116283, 81353, 80610, 142522, 64946, 125652,
        61688, 58367, 118930, 89711, 115239, 66403, 92405, 114593, 112818, 75964, 126093, 139781,
        144801, 88725, 125958, 116869, 119676,
    ];

    #[test]
    fn test_part_1() {
        let mut running_sum = 0;
        for module in INPUT.iter() {
            running_sum = running_sum + fuel_for_mass(*module);
        }

        assert_eq!(3429947, running_sum);
    }

    #[test]
    fn test_fuel_for_module_and_fuel() {
        assert_eq!(2, fuel_for_module_and_fuel(14));
        assert_eq!(966, fuel_for_module_and_fuel(1969));
        assert_eq!(50346, fuel_for_module_and_fuel(100756));
    }

    #[test]
    fn test_part_2() {
        let mut running_sum = 0;
        for module in INPUT.iter() {
            running_sum = running_sum + fuel_for_module_and_fuel(*module);
        }

        assert_eq!(5142043, running_sum);
    }
}
