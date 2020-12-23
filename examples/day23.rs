fn main() {
    // let input = "685974213";
    let input = "389125467";
    let mut input: Vec<i32> = input
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    let max = *input.iter().max().unwrap();

    let new_max = 20;
    for i in max..=new_max {
        input.push(i);
    }
    let max = new_max;

    for i in 0..100 {
        if i % 10000 == 0 {
            println!("{}", i);
        }
        let current = *input.iter().nth(0).unwrap();
        let mut target = current - 1;
        if target == 0 {
            target = max;
        }
        let remove = input.iter().skip(1).take(3).cloned().collect::<Vec<_>>();

        while remove.contains(&target) {
            target -= 1;
            if target == 0 {
                target = max;
            }
        }
        let target_pos = input.iter().position(|x| *x == target).unwrap();
        // println!("remove: {:?} target {}", remove, target);

        let skip_to_insert = target_pos - 4;
        let pre = input.iter().skip(4).take(skip_to_insert + 1);
        let post = input
            .iter()
            .skip(4 + skip_to_insert + 1)
            .cloned()
            .chain(std::iter::once(current));
        let new: Vec<_> = pre.chain(remove.iter()).cloned().chain(post).collect();

        println!("new:\n{:?} -> {:?}", input, new);
        input = new;
    }
    // .chain(remove.iter()).
}
