#[derive(Debug, Clone)]
struct Cmd {
    command: String, 
    occurrence: i32 
}

fn main(){
    // let commands_str: String = String::from("commandA\ncommandB\ncommandC\ncommandB\n");
    //     let command = Cmd {
    //         command: String::from("commandC"),
    //         occurrence: 0
    //     };

    //     let vec: Vec<Cmd> = commands_str.split("\n").map(|c| {
    //         let cmd = Cmd {
    //             command: c.to_string(),
    //             occurrence: 1
    //         };
    //         return cmd;
    //     }).collect();
    //     assert_eq!(2, get_index_of(command, &vec));
    // let commands_str: String = String::from("commandA\n
    //     commandB\n
    //     commandC\n
    //     commandA\n
    //     commandB\n
    //     commandC\n
    //     commandD\n
    //     commandA\n
    //     commandB\n");

    //     let ocurr: Vec<Cmd> = vec![];

    //let mut index: i32 = -1;

    //for c in commands_str.split("\n") {
    //    let cmd = Cmd {
    //        command: c.to_string(),
    //        occurrence: 1
    //    };

    //    index = get_index_of(cmd, &ocurr);
    //    // let mut exists: Vec<Cmd> = ocurr.clone().into_iter()
    //    //     .filter(|cmd| cmd.command == c.to_string())
    //    //     .collect();

    //    //if exists.len() == 1{
    //    //    //increment occ
    //    //}else{
    //    //    ocurr.push(exists.unwrap().pop());
    //    //}

    //}
    //println!("{:?}", index);
}

fn get_index_of(element: Cmd, in_vector: &Vec<Cmd>) -> i32{
    let mut i = 0;
    let mut index: i32 = -1;
    for el in in_vector.iter() {
        if el.command == element.command {
           index = i; 
           break;
        }
        i += 1;
    }
    return index;
}


