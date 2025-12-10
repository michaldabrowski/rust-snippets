use std::fs::read_to_string;

fn main() {
    println!("------------ 2025 Day 10 ------------");
    let input = read_to_string("input.txt").expect("Failed to read input file");

    let mut total_presses = 0;
    // let mut total_presses_part2 = 0;

    for line in input.lines() {
        let bracket_start = line.find('[').unwrap();
        let bracket_end = line.find(']').unwrap();
        let diagram = &line[bracket_start + 1..bracket_end];

        let target: Vec<bool> = diagram.chars().map(|c| c == '#').collect();
        let num_lights = target.len();

        let curly_start = line.find('{').unwrap();
        // let curly_end = line.find('}').unwrap();
        let buttons_section = &line[bracket_end + 1..curly_start];
        let buttons: Vec<Vec<usize>> = parse_buttons_section(buttons_section);

        // part 1
        let min_presses = find_min_presses(&buttons, &target, num_lights);
        total_presses += min_presses;

        // part 2
        // let joltage_section = &line[curly_start + 1..curly_end];
        // let joltage_target: Vec<usize> = joltage_section
        //     .split(',')
        //     .map(|s| s.trim().parse().unwrap())
        //     .collect();

        // let min_presses_p2 = find_min_presses_joltage(&buttons, &joltage_target);
        // TODO efficient solution for part 2
    }

    println!("{}", total_presses);
    // println!("{}", total_presses_part2);
}

fn parse_buttons_section(section: &str) -> Vec<Vec<usize>> {
    let mut buttons: Vec<Vec<usize>> = Vec::new();
    let mut in_parens = false;
    let mut current_nums: Vec<usize> = Vec::new();
    let mut num_str = String::new();

    for c in section.chars() {
        match c {
            '(' => {
                in_parens = true;
                current_nums.clear();
                num_str.clear();
            }
            ')' => {
                if !num_str.is_empty() {
                    current_nums.push(num_str.parse().unwrap());
                    num_str.clear();
                }
                buttons.push(current_nums.clone());
                in_parens = false;
            }
            ',' if in_parens => {
                if !num_str.is_empty() {
                    current_nums.push(num_str.parse().unwrap());
                    num_str.clear();
                }
            }
            '0'..='9' if in_parens => {
                num_str.push(c);
            }
            _ => {}
        }
    }

    buttons
}

fn find_min_presses(buttons: &[Vec<usize>], target: &[bool], num_lights: usize) -> usize {
    let num_buttons = buttons.len();
    let mut min_presses = usize::MAX;

    let mut pressed = vec![false; num_buttons];

    loop {
        let mut state = vec![false; num_lights];
        let mut presses = 0;

        for (i, &is_pressed) in pressed.iter().enumerate() {
            if is_pressed {
                for &light_idx in &buttons[i] {
                    state[light_idx] = !state[light_idx];
                }
                presses += 1;
            }
        }

        if state == *target && presses < min_presses {
            min_presses = presses;
        }

        // Increment to next subset (binary counting with booleans)
        let mut carry = true;
        for p in pressed.iter_mut() {
            if carry {
                if *p {
                    *p = false; // 1 + 1 = 0, carry 1
                } else {
                    *p = true; // 0 + 1 = 1, no carry
                    carry = false;
                }
            }
        }

        // If carry is still true, we've tried all 2^n combinations
        if carry {
            break;
        }
    }

    if min_presses == usize::MAX {
        0
    } else {
        min_presses
    }
}
