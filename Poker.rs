fn main(){
    // println!("hello world");
    // let mut deck:[i32; 10]=[1,2,3,4,5,6,7,8,9,10]; //sample deck
    // //println!("{}", check_sequence_helper([1,2,3,4,6]));
    // return_card_frequency_helper([1,3,3,3,3],3);
    // println!("here {}",check_flush([1,2,3,4,5]));
    // println!("here2 {}",check_three_of_a_kind([1,2,2,2,2]));

    println!("here {:?}",get_card_value(26));
}


fn deal(deck: &mut [i32;10]) ->[i32;5] {
    let mut hand1=[deck[0],deck[2],deck[4],deck[6],deck[8]];
    let mut hand2=[deck[1],deck[3],deck[5],deck[7],deck[9]];
    let mut winner_deck=determine_winner(hand1,hand2);
    return winner_deck; //TODO print
}

fn get_card_value(card:i32)->i32{ //returns the card value in the range 0-12
    if card%13==1{
        (card+11)%13
    }
    else {(card-2)%13}
}

fn get_card_suite(card:i32)->i32{ //returns the card suite from 0 to 3
    (card-1)/13
}

fn check_suites_helper(hand:[i32;5])->bool{ //a helper method that returns true is all suites are the same
    let reference_suite=get_card_suite(hand[0]);
    for index in 1..5{ //5, not 4 as it works as range() in Python!
        if get_card_suite(hand[index])!=reference_suite{
            return false;
        }
    }
    return true;
}

fn check_sequence_helper(hand:[i32;5])->bool{ //checks if the cards are in sequence
    let reduced_hand=sort_hand(convert_hand(hand));
    println!("reduced hand is {:?}",reduced_hand);
    for index in 0..4{
        if reduced_hand[index+1]-reduced_hand[index]!=1{
            return false;
        }
    }
    return true;
}

fn return_card_frequency_helper(hand:[i32;5],number:i32)->bool{ //returns if there are than many cards
    let mut frequences=[0,0,0,0,0];
    let converted_hand=convert_hand(hand); //the hand has to be converted
    for card in 0..5{
        for index in 0..5{
            if converted_hand[index]==converted_hand[card]{
                frequences[card]+=1;
            }
        }
    }
    println!("{:?}", frequences);

    if number==2{  //special case for 2 pairs
        let mut vec = frequences.to_vec(); //make a vector of frequences
        vec.sort();
        return vec==vec![1,2,2,2,2];
    }

    else if number==1{ //special case for a pair
        let mut vec = frequences.to_vec(); //make a vector of frequences
        vec.sort();
        return vec==vec![1,1,1,2,2];
    }

    //cases for 3 and 4 are defined below:
    if frequences.iter().any(|&x| x == number){ //checks if any number is equal to the desired one
        println!("{}", true);
        return true;
    }
    println!("{}", false);
    return false;
}

fn convert_hand(hand:[i32;5])->[i32;5]{// converts all cards to 0-12 values
    let mut converted_hand=[0,0,0,0,0];
    for index in 0..5{
        converted_hand[index]=get_card_value(hand[index]);
    }
    // if sort_hand(converted_hand)==[0,1,2,3,12]{
    //     return []
    // }
    return converted_hand;
}

fn sort_hand(hand:[i32;5])->[i32;5]{ //sorts the hand in increasing order
    let mut sorted_hand=hand;
    for i in 0..5{
        for index in 0..4{
            if sorted_hand[index]>=sorted_hand[index+1]{
                    let temp=sorted_hand[index+1];
                    sorted_hand[index+1]=sorted_hand[index];
                    sorted_hand[index]=temp;
            }
        }
    }
    println!("sorted hand is {:?}",sorted_hand);
    return sorted_hand;
}
fn check_royal_flush(hand:[i32;5])->bool{ //checks the Royal Flush
    if check_suites_helper(hand)&&sort_hand(convert_hand(hand))==[8,9,10,11,12]{
        return true;
    }
    return false;
}

fn check_straight_flush(hand:[i32;5])->bool{ //checks Straight Flush
    if check_sequence_helper(hand)&&check_suites_helper(hand){
        return true;
    }
    return false;
}

fn check_four_of_a_kind(hand:[i32;5])->bool{ //there are 4 identical cards...
    if return_card_frequency_helper(hand,4)==true{
        return true;
    }
    return false;
}

fn check_fullhouse(hand:[i32;5])->bool{//there are 3 identical cards and a pair
    if return_card_frequency_helper(hand,3)&&return_card_frequency_helper(hand,2){
        return true;
    }
    return false;
}

fn check_flush(hand:[i32;5])->bool{ // if all cards have the same suit
    return check_suites_helper(hand);
}

fn check_straight(hand:[i32;5])->bool{ // in sequence but not in the same suite
    if !check_suites_helper(hand)&&check_sequence_helper(hand){
        return true;
    }
    return false;
}

fn check_three_of_a_kind(hand:[i32;5])->bool{ //there are only 3 cards and no pairs
    if return_card_frequency_helper(hand,3)&&!return_card_frequency_helper(hand,2){
        return true;
    }
    return false;
}

fn check_two_pairs(hand:[i32;5])->bool{
    return return_card_frequency_helper(hand,2);
}

fn check_pair(hand:[i32;5])->bool{
    return return_card_frequency_helper(hand,1);
}

fn get_high_card(hand:[i32;5])->i32{ //returns a high card
    let mut converted_hand=convert_hand(hand); //obtain a hand in range from 0 to 12
    let mut max=-1;
    //case when ace is not high
    if check_straight(hand){
        for index in 0..5{
            if max<converted_hand[index]&&converted_hand[index]!=12{
                max=converted_hand[index];
            }
        }
        return max;
    }
    //regular case
    for index in 0..5{
        if max<converted_hand[index]{
            max=converted_hand[index];
        }
    }
    return max;
}


fn determine_rank(hand:[i32;5])->i32{ //determines the rank of a hand
    if check_royal_flush(hand){
        return 10;
    }
    else if check_straight_flush(hand){
        return 9;
    }
    else if check_four_of_a_kind(hand){
        return 8;
    }
    else if check_fullhouse(hand){
        return 7;
    }
    else if check_flush(hand){
        return 6;
    }
    else if check_straight(hand){
        return 5;
    }
    else if check_three_of_a_kind(hand){
        return 4;
    }
    else if check_two_pairs(hand){
        return 3;
    }
    else if check_pair(hand){
        return 2;
    }
    return 1;
}

fn determine_winner(hand1:[i32;5],hand2:[i32;5])->[i32;5]{ //determines the winner
    let rank1=determine_rank(hand1);
    let rank2=determine_rank(hand2);
    if rank1>rank2{
        return hand1;
    }
    if rank1<rank2{
        return hand2;
    }
    //TODO Tiebreakers should go here
    let highcard1=get_high_card(hand1);
    let highcard2=get_high_card(hand2);
    if highcard1>highcard2{
        return hand1;
    }
    if highcard1<highcard2{
        return hand2;
    }
    let highsuit1=get_card_suite(highcard1);
    let highsuit2=get_card_suite(highcard2);
    if highsuit1>highsuit2{
        return hand1;
    }
    else{
        return hand2;
    }
}



