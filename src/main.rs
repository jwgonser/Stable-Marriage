//#![feature(rustc_private)]
extern crate rand;

use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;
use std::hash::Hash;
use std::hash::Hasher;

use rand::prelude::*;

trait MarriageItem<T> : Hash {

    fn generate_preferences(&self, list : Vec<T>) -> Vec<T>;

}

#[derive(Clone)]
struct MIContainer <A : MarriageItem<B> + PartialEq, B : MarriageItem<A> + PartialEq>{
    currently_matched : bool,
    item : A,
    position : usize,
    preference_list : Vec<B>,


}


#[derive(Clone, Hash, PartialEq)]
struct Person {

    name : String,

}

impl Person {

    fn new(name : String) -> Person {
        let p = Person {
            name,
        };

        return p;
    }

    

}

impl MarriageItem<Person> for Person{

    fn generate_preferences(&self, list : Vec<Person>) -> Vec<Person>{
        let mut list2 = list.clone();
        let mut pref_list = Vec::new();
        let l = list.len();
        let mut rng = thread_rng();

        for _ in 0..l {
            let n : usize = rng.gen_range(0, list2.len());
            pref_list.push(list2.remove(n));
        }

        return pref_list;
    }

}


// main function used for testing
fn main(){

    let list_of_men = get_lists("src/men.txt".to_string());
    let list_of_women = get_lists("src/women.txt".to_string());
    let mut men = Vec::new();
    let mut women = Vec::new();


    for x in list_of_men.clone() {
        men.push(Person::new(x));
    }

    for x in list_of_women {
        women.push(Person::new(x));
    }

    let male_pairs = marry(men.clone(), women.clone());
    let female_pairs = marry(women, men);
    

    println!("male_pairs:");
    for x in 0..male_pairs.len() {
        print!("[{:?}, {:?}]  ", male_pairs[x].0.name, male_pairs[x].1.name);
    }

    println!("\nfemale_pairs:");
    for x in 0..female_pairs.len() {
        print!("[{:?}, {:?}]  ", female_pairs[x].0.name, female_pairs[x].1.name);
    }

}


/// This is the function that implements the Stable Marriage Algorithm.
/// Accepts generic objects contained inside of a MIContainer (Marriage Item Container),
/// and using a provided preference function, creates preference lists for each item in
/// both proposers and proposees. Then, based on those generated preferences, will find one 
/// of the most stable pairing configurations.
fn marry<A, B>(proposers : Vec<A>, proposees : Vec<B> ) -> Vec<(A,B)>
    where A:MarriageItem<B> + Clone + PartialEq,
          B:MarriageItem<A> + Clone + PartialEq{

    // collect all proposers into generic containers
    let mut proposers2 : Vec<MIContainer<A,B>> = proposers.clone().iter().map(|proposer|{
        MIContainer{
            currently_matched : false,
            item : proposer.clone(),
            position : 0,
            preference_list : Vec::new(),
        }
    }).collect();

    // collect all proposees into generic containers
    let mut proposees2 : Vec<MIContainer<B,A>> = proposees.clone().iter().map(|proposee|{
        MIContainer{
            currently_matched : false,
            item : proposee.clone(),
            position : 0,
            preference_list : Vec::new(),
        }
    }).collect();

    // create proposers' preference lists
    for mut x in proposers2.iter_mut() {
        x.preference_list = x.item.generate_preferences(proposees.clone());
    }

    // create proposees' preference lists
    for mut x in proposees2.iter_mut() {
        x.preference_list = x.item.generate_preferences(proposers.clone());
    }

    // create list of pairs and algorithm completed variable
    let mut marriage_list = Vec::new();
    let mut complete = false;
    
    // begin marriage
     while !complete {
        complete = true;
        let mut unmatched = Vec::new();
        // iterate through the proposers
        for p in proposers2.iter_mut() {
            // if a proposer has no match, attempt to match
            if p.currently_matched == false {
                // iterate through all the proposees
                for i in proposees2.iter_mut() {
                    // find proposee 
                    if p.preference_list[p.position] == i.item {
                        let mut rank = <i32>::max_value();
                        // set rank to proposee's preference value
                        for x in 0..i.preference_list.len() {
                            
                            if i.preference_list[x] == p.item {
                                rank = x as i32 ;
                                break
                            }
                        }
                        // if proposee is free, match
                        if i.currently_matched == false {
                            p.currently_matched = true;
                            i.currently_matched = true;
                            i.position = rank as usize;
                        }
                        // if proposee is matched, but proposer has higher rank, rematch
                        else if i.currently_matched == true && i.position < rank as usize {
                            p.currently_matched = true;
                            // proposee dumps worse option
                            unmatched.push(i.preference_list[i.position].clone());
                            i.position = rank as usize;
                        }
                        // if proposer is outright rejected, place into unmatched
                        else{
                            unmatched.push(p.item.clone());
                        }
                        break
                    }
                }
            }
        }
    
        

        //check to see if any unmatched remain
        if unmatched.len() > 0 {
            complete = false;
            // look at all unmatched
            for x in unmatched {
                // find ummatched proposers
                for p in proposers2.iter_mut() {
                    if p.item == x {
                        p.position = p.position + 1;
                        p.currently_matched = false;
                        break
                    }
                }
            }
        }
    
    }

    for x in proposers2 {
        marriage_list.push((x.item, x.preference_list[x.position].clone()));
    }

    return marriage_list;
}


fn get_lists(filename : String) -> Vec<String> {

    match File::open(filename) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents);
            let split = contents.split("\r\n");
            let list : Vec<String> = split.map(|s| s.to_string()).collect();
            return list;
        }
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    }

}
