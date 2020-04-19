fn main(){
    // println!("hello world");
    // let mut deck:[i32; 10]=[1,2,3,4,5,6,7,8,9,10]; //sample deck
    // //println!("{}", check_sequence_helper([1,2,3,4,6]));
    // return_card_frequency_helper([1,3,3,3,3],3);
    // println!("here {}",check_flush([1,2,3,4,5]));
    // println!("here2 {}",check_three_of_a_kind([1,2,2,2,2]));

    //println!("here {:?}",format_output([1,10,11,12,13]));

    test(); //runs the test cases

}


fn deal(deck: &mut [i32;10]) ->[i32;5] {
    let mut hand1=[deck[0],deck[2],deck[4],deck[6],deck[8]];
    let mut hand2=[deck[1],deck[3],deck[5],deck[7],deck[9]];
    let mut winner_deck=determine_winner(hand1,hand2);
    winner_deck 
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
    if reduced_hand==[0,1,2,3,12]{ //the case when ace is low(straight)
        return true;
    }
    //println!("reduced hand is {:?}",reduced_hand);
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
    //println!("{:?}", frequences);

    if number==2{  //special case for 2 pairs
        let mut vec = frequences.to_vec(); //make a vector of frequences
        vec.sort();
        return vec==vec![1,2,2,2,2];
    }

    else if number==1{ //special case for a pair
        let mut vec = frequences.to_vec(); //make a vector of frequences
        vec.sort();
        return vec==vec![1,1,1,2,2]||vec==vec![2,2,3,3,3];
    }

    //cases for 3 and 4 are defined below:
    if frequences.iter().any(|&x| x == number){ //checks if any number is equal to the desired one
        //println!("{}", true);
        return true;
    }
    //println!("{}", false);
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
    //println!("sorted hand is {:?}",sorted_hand);
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
    if return_card_frequency_helper(hand,3)&&return_card_frequency_helper(hand,1){
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
    let mut converted_hand=sort_hand(convert_hand(hand)); //obtain a hand in range from 0 to 12
    //println!("{:?} converted hand",converted_hand);
    if converted_hand==[0,1,2,3,12]{ //the only case when ace is low
        return 3;
    }
    else{
        return converted_hand[4];
    }
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
    return determineTieBreaker(hand1,hand2,rank1);

    /*
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
    */
}


fn format_output(hand:[i32;5])->[String;5]{ //returns a formatted winning hand
    let mut result: [String; 5] = Default::default(); //a list holding 5 String objects

    for index in 0..5{
        //concatinates the card value from 1-13 and a String suit
        result[index]=format!("{}{}", String::from(return_card_value_helper(hand[index])), String::from(return_string_suit_helper(hand[index])));
    }


    let mut temp=String::new(); //holds a value for swap
    //sorts a list in ascending order
    for i in 0..5{
        for j in 0..4{
            if result[j+1]<result[j]{
                temp=result[j+1].clone();
                result[j+1]=result[j].clone();
                result[j]=temp.clone();
            }
        }
    }

    return result;
}


fn return_string_suit_helper(card_number:i32)->String{ //returns a string suit
    if card_number<=13{
        return "C".to_string();
    }
    if card_number<=26{
        return "D".to_string();
    }
    if card_number<=39{
        return "H".to_string();
    }
    
    return "S".to_string();
}

fn return_card_value_helper(card_number:i32)->String{ //returns a card value in the range from 1 to 13
    if card_number>13{
        return return_card_value_helper(card_number-13);
    }
    else{
        return card_number.to_string();
    }
}

fn test(){
    //TODO Modify both lists and add proper test cases
    let mut test_cases:[[i32;10];86]=[[14, 10, 17, 11, 15, 7, 16, 9, 18, 8],[16, 10, 18, 8, 14, 9, 17, 7, 15, 11],[16, 49, 14, 9, 17, 35, 15, 22, 18, 48],[16, 9, 14, 48, 15, 35, 18, 22, 17, 49],[23, 9, 49, 2, 10, 48, 50, 22, 36, 35],[49, 35, 23, 22, 50, 2, 10, 48, 36, 9],[48, 40, 34, 42, 21, 44, 8, 45, 35, 50],[35, 42, 34, 44, 21, 40, 8, 45, 48, 50],[5, 34, 2, 31, 4, 29, 3, 28, 7, 30],[4, 30, 2, 34, 3, 28, 5, 29, 7, 31],[2, 47, 3, 10, 31, 46, 32, 24, 17, 48],[32, 10, 31, 48, 2, 46, 3, 24, 17, 47],[28, 12, 15, 11, 2, 13, 30, 26, 29, 39],[28, 39, 2, 11, 15, 13, 30, 26, 29, 12],[22, 13, 16, 26, 30, 25, 43, 2, 3, 12],[3, 13, 30, 25, 43, 2, 22, 26, 16, 12],[14, 41, 15, 52, 1, 27, 2, 28, 51, 40],[1, 41, 15, 27, 2, 40, 51, 28, 14, 52],[23, 46, 41, 42, 10, 15, 48, 2, 44, 50],[44, 50, 23, 15, 41, 46, 48, 2, 10, 42],[7, 51, 6, 47, 8, 49, 13, 28, 41, 29],[41, 29, 13, 51, 6, 28, 8, 49, 7, 47],[41, 29, 12, 49, 6, 47, 8, 28, 7, 52],[12, 47, 7, 49, 6, 29, 8, 28, 41, 52],[47, 40, 20, 52, 7, 51, 46, 49, 33, 50],[7, 50, 46, 52, 47, 51, 33, 40, 20, 49],[17, 47, 18, 22, 16, 35, 15, 9, 14, 49],[17, 9, 15, 49, 14, 22, 18, 47, 16, 35],[14, 9, 17, 32, 16, 22, 18, 47, 15, 49],[14, 47, 17, 9, 15, 22, 18, 32, 16, 49],[17, 26, 16, 52, 18, 51, 14, 39, 15, 38],[15, 52, 16, 38, 14, 26, 18, 39, 17, 51],[10, 42, 23, 45, 49, 40, 50, 47, 36, 44],[23, 44, 49, 40, 36, 45, 50, 42, 10, 47],[34, 16, 48, 50, 21, 29, 8, 42, 35, 45],[8, 16, 48, 42, 34, 50, 21, 29, 35, 45],[50, 41, 23, 42, 10, 40, 36, 31, 49, 30],[50, 41, 36, 42, 10, 30, 23, 40, 49, 31],[31, 25, 32, 26, 2, 13, 30, 4, 3, 12],[2, 26, 31, 25, 30, 4, 3, 13, 32, 12],[30, 26, 31, 13, 32, 24, 2, 12, 3, 4],[30, 24, 2, 12, 32, 4, 3, 13, 31, 26],[2, 24, 3, 22, 31, 13, 30, 4, 32, 12],[31, 4, 30, 13, 2, 22, 3, 12, 32, 24],[3, 13, 1, 26, 5, 11, 9, 12, 7, 39],[3, 13, 1, 11, 9, 26, 5, 12, 7, 39],[3, 2, 1, 12, 5, 13, 7, 25, 9, 26],[5, 13, 7, 2, 1, 25, 3, 26, 9, 12],[3, 26, 1, 20, 9, 2, 5, 12, 7, 13],[9, 2, 5, 26, 7, 12, 1, 13, 3, 20],[6, 50, 7, 40, 41, 49, 8, 51, 13, 52],[8, 51, 41, 40, 6, 50, 13, 52, 7, 49],[6, 52, 7, 51, 26, 50, 13, 49, 8, 40],[7, 40, 13, 50, 6, 52, 26, 51, 8, 49],[21, 51, 8, 50, 26, 52, 13, 40, 6, 49],[8, 50, 6, 51, 26, 52, 21, 49, 13, 40],[21, 50, 26, 40, 34, 52, 13, 51, 8, 49],[26, 52, 34, 40, 13, 50, 8, 49, 21, 51],[17, 10, 15, 7, 14, 8, 16, 11, 18, 9],[18, 8, 14, 11, 17, 10, 15, 9, 16, 7],[15, 22, 18, 49, 17, 48, 16, 9, 14, 35],[14, 35, 18, 49, 16, 22, 17, 48, 15, 9],[17, 47, 3, 48, 32, 46, 31, 10, 2, 24],[31, 46, 17, 48, 2, 47, 32, 24, 3, 10],[13, 49, 8, 28, 41, 51, 7, 29, 6, 47],[8, 49, 13, 47, 7, 28, 6, 29, 41, 51],[16, 38, 15, 52, 18, 51, 14, 39, 17, 26],[17, 51, 14, 52, 16, 39, 15, 26, 18, 38],[17, 9, 18, 8, 16, 10, 14, 11, 15, 7],[16, 10, 15, 9, 17, 11, 18, 8, 14, 7],[18, 48, 15, 22, 17, 49, 14, 9, 16, 35],[14, 49, 18, 22, 16, 35, 17, 9, 15, 48],[3, 39, 2, 13, 30, 12, 32, 26, 31, 11],[2, 11, 30, 39, 31, 13, 32, 26, 3, 12],[33, 49, 46, 51, 47, 52, 20, 40, 7, 50],[47, 52, 33, 40, 20, 50, 7, 49, 46, 51],[17, 26, 15, 38, 18, 39, 16, 51, 14, 52],[18, 38, 16, 52, 15, 26, 14, 39, 17, 51],[32, 26, 2, 12, 30, 4, 31, 13, 3, 25],[2, 4, 3, 13, 32, 26, 30, 25, 31, 12],[32, 4, 2, 24, 30, 13, 3, 12, 31, 26],[3, 26, 32, 24, 2, 13, 30, 12, 31, 4],[3, 13, 32, 12, 31, 22, 30, 4, 2, 24],[3, 13, 32, 12, 31, 4, 2, 22, 30, 24],[34, 40, 26, 52, 13, 50, 21, 51, 8, 49],[26, 52, 21, 49, 8, 51, 13, 50, 34, 40]];
    let correct_result=[["10C", "11C", "7C", "8C", "9C"],["10C", "11C", "7C", "8C", "9C"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["10C", "10D", "10H", "10S", "11S"],["10C", "10D", "10H", "10S", "11S"],["8C", "8D", "8H", "9H", "9S"],["8C", "8D", "8H", "9H", "9S"],["2H", "3H", "4H", "5H", "8H"],["2H", "3H", "4H", "5H", "8H"],["10C", "11D", "7S", "8S", "9S"],["10C", "11D", "7S", "8S", "9S"],["11C", "12C", "13C", "13D", "13H"],["11C", "12C", "13C", "13D", "13H"],["12C", "12D", "13C", "13D", "2C"],["12C", "12D", "13C", "13D", "2C"],["13S", "1H", "1S", "2H", "2S"],["13S", "1H", "1S", "2H", "2S"],["10C", "10D", "2S", "5S", "9S"],["10C", "10D", "2S", "5S", "9S"],["13C", "2S", "6C", "7C", "8C"],["13C", "2S", "6C", "7C", "8C"],["10S", "13S", "2H", "3H", "8S"],["10S", "13S", "2H", "3H", "8S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["10C", "10D", "10H", "10S", "11S"],["10C", "10D", "10H", "10S", "11S"],["8C", "8D", "8H", "9H", "9S"],["8C", "8D", "8H", "9H", "9S"],["10C", "10D", "10H", "10S", "11S"],["10C", "10D", "10H", "10S", "11S"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["1C", "3C", "5C", "7C", "9C"],["1C", "3C", "5C", "7C", "9C"],["1C", "3C", "5C", "7C", "9C"],["1C", "3C", "5C", "7C", "9C"],["1C", "3C", "5C", "7C", "9C"],["1C", "3C", "5C", "7C", "9C"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["10C", "11C", "7C", "8C", "9C"],["10C", "11C", "7C", "8C", "9C"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["10C", "11D", "7S", "8S", "9S"],["10C", "11D", "7S", "8S", "9S"],["13C", "2S", "6C", "7C", "8C"],["13C", "2S", "6C", "7C", "8C"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["10C", "11C", "7C", "8C", "9C"],["10C", "11C", "7C", "8C", "9C"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"],["1D", "2D", "3D", "4D", "5D"],["1D", "2D", "3D", "4D", "5D"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["2C", "3C", "4H", "5H", "6H"],["10S", "11S", "12S", "13S", "1S"],["10S", "11S", "12S", "13S", "1S"]];
    for i in 0..test_cases.len(){
        let result=format_output(deal(&mut test_cases[i])); //TODO uncomment this line to tell the overall program
        //let result=format_output(test_cases[i]); //TODO comment this line out
        if result!=correct_result[i]{
            println!("Case-{:?} Correct Result-{:?} Incorrect Result/Output-{:?}",test_cases[i],correct_result[i],result);
        }else {
            println!("correct");
        }
    }
}












//--------------------------------- tie breaker functions---------------------------

fn determineTieBreaker(hand1:[i32;5],hand2:[i32;5],rank:i32)->[i32;5]{
    if rank==10 {
        return tieBreakRoyalFlush(hand1,hand2);//
    }
    else if rank==9 {
        return tieBreakStraightFlush(hand1,hand2);//9;
    }
    else if rank==8 {
        return tieBreakFour(hand1,hand2);//8;
    }
    else if rank==7 {
        return tieBreakFull(hand1,hand2);//7;
    }
    else if rank==6 {
        return tieBreakFlush(hand1,hand2);//6;
    }
    else if rank==5 {
        return tieBreakStraight(hand1,hand2);//5;
    }
    else if rank==4 {
        return tieBreakThree(hand1,hand2);//4;
    }
    else if rank==3 {
        return tieBreakTwoPair(hand1,hand2);//3;
    }
    else if rank==2 {
        return tieBreakPair(hand1,hand2);//2;
    }
    return tieBreakHigh(hand1,hand2);
    
}


fn tieBreakRoyalFlush(hand1:[i32;5],hand2:[i32;5])->[i32;5]{
    if( get_card_suite(hand1[0]) > get_card_suite(hand2[0]) ){
        return hand1;
    }else {
        return hand2;
    }
}

fn tieBreakStraightFlush(hand1:[i32;5],hand2:[i32;5])->[i32;5]{
    tieBreakStraight(hand1,hand2)
}

fn tieBreakFour(hand1:[i32;5],hand2:[i32;5])->[i32;5]{
    if ( getValueOfRepeat(hand1,4) > getValueOfRepeat(hand2,4)){
        return hand1;
    } else {
        return hand2;
    }
}

fn tieBreakFull(hand1:[i32;5],hand2:[i32;5])->[i32;5]{
    tieBreakThree(hand1,hand2)
}
fn tieBreakFlush(hand1:[i32;5],hand2:[i32;5])->[i32;5]{
    tieBreakHigh(hand1,hand2)
}
fn tieBreakStraight(hand1:[i32;5],hand2:[i32;5])->[i32;5]{
    let hand1Low = (doesHandContain(hand1, 12) && doesHandContain(hand1,0) );
    let hand2Low = (doesHandContain(hand2, 12) && doesHandContain(hand2,0) );  
    if( hand1Low && hand2Low  ){
        //suit compare of 5 (which has a value of 3)
        if( getSuitOfFaceVal(hand1,3) > getSuitOfFaceVal(hand1,3) ){
            return hand1;
        } else {
            return hand2;
        }
    } else if (hand1Low){
        return hand2;
    }else if(hand2Low){
        return hand1;
    }

    //ace is high
    if( getHighestFaceVal(hand1) > getHighestFaceVal(hand2)){
        return hand1;
    } else if (getHighestFaceVal(hand2) > getHighestFaceVal(hand1)){
        return hand2;
    } else if( getSuitOfHighestFaceVal(hand1) > getSuitOfHighestFaceVal(hand2)){
        return hand1;
    }else {
        return hand2;
    }
}
fn tieBreakThree(hand1:[i32;5],hand2:[i32;5])->[i32;5]{
    if ( getValueOfRepeat(hand1,3) > getValueOfRepeat(hand2,3)){
        return hand1;
    } else {
        return hand2;
    }
}

fn tieBreakTwoPair(hand1:[i32;5],hand2:[i32;5])->[i32;5]{
    let hand1Sort = sortHandOnValue(hand1);
    let hand2Sort = sortHandOnValue(hand2);
    let high1 = get_card_value(hand1Sort[3]);
    let low1 = get_card_value(hand1Sort[1]);
    let high2 = get_card_value(hand2Sort[3]);
    let low2 = get_card_value(hand2Sort[1]);

    if(high1>high2){
        return hand1;
    } else if( high2>high1){
        return hand2;
    } else if(low1>low2){
        return hand1;
    } else if(low2>low1){
        return hand2;
    }
    //compare kicker
    let kicker1 = getValueOfRepeat(hand1, 1);
    let kicker2 = getValueOfRepeat(hand2,1);
    if(kicker1>kicker2){
        return hand1;
    } else if (kicker2>kicker1){
        return hand2;
    }
    //compare suit of high pair
    if( twoPairHelperSuitOfHighPair(hand1Sort) > twoPairHelperSuitOfHighPair(hand2Sort)){
        return hand1;
    }
    return hand2;
}

fn twoPairHelperSuitOfHighPair(handSort:[i32;5])->i32{
    let faceVal = get_card_value(handSort[3]);
    let mut maxSuit = get_card_suite(handSort[3]);
    if faceVal == get_card_value(handSort[2]){
        maxSuit = max(maxSuit, get_card_suite(handSort[2]));
    }
    if faceVal == get_card_suite(handSort[4]){
        maxSuit = max(maxSuit, get_card_suite(handSort[4]));
    }
    return maxSuit;
}

fn tieBreakPair(hand1:[i32;5],hand2:[i32;5])->[i32;5]{
    let pair1 = getValueOfRepeat(hand1, 2);
    let pair2 = getValueOfRepeat(hand2, 2);
    if( pair1>pair2){
        return hand1;
    } else if(pair2>pair1){
        return hand2;
    }

    //compare all Values 
    let hand1Sort = sortHandOnValue(hand1);
    let hand2Sort = sortHandOnValue(hand2);
    for index in (0..5).rev(){
        if( get_card_value(hand1Sort[index]) > get_card_value(hand2Sort[index]) ){
            return hand1; 
        } else if ( get_card_value(hand2Sort[index]) > get_card_value(hand1Sort[index]) ){
            return hand2;
        }
    }

    //compare suit of pair
    if( pairSuit(hand1,pair1) > pairSuit(hand2,pair2)){
        return hand1;
    }
    return hand2;
}

fn pairSuit(hand:[i32;5],pairVal:i32)->i32{
    let mut maxSuit = -1;
    for card in &hand{
        if get_card_value(*card) == pairVal {
            maxSuit = max( get_card_suite(*card), maxSuit);
        }
    }
    return maxSuit;
}


fn tieBreakHigh(hand1:[i32;5],hand2:[i32;5])->[i32;5]{
    let hand1Sort = sortHandOnValue(hand1);
    let hand2Sort = sortHandOnValue(hand2);
    for index in (0..5).rev(){
        if( get_card_value(hand1Sort[index]) > get_card_value(hand2Sort[index]) ){
            return hand1; 
        } else if ( get_card_value(hand2Sort[index]) > get_card_value(hand1Sort[index]) ){
            return hand2;
        }
    }
    if ( get_card_suite(hand1Sort[4]) > get_card_suite(hand2Sort[4]) ){//compare suit of high cards 
        return hand1;
    } else {
        return hand2;
    }  
}


//helper functions

//faceVal from 0-12
fn doesHandContain(hand:[i32;5],faceVal:i32)->bool{
    for ele in &hand {
        if( get_card_value(*ele) == faceVal){
            return true;
        }
    }
    return false;
}

fn getHighestFaceVal(hand:[i32;5])->i32{
    let mut highest = get_card_value(hand[0]);
    for card in &hand{
        if( get_card_value(*card) > highest){
            highest = get_card_value(*card);
        }
    }
    return highest;
}

fn getSuitOfHighestFaceVal(hand:[i32;5])->i32{
    let mut highest = get_card_value(hand[0]);
    let mut suite = get_card_suite(hand[0]);
    for card in &hand{
        if( get_card_value(*card) > highest){
            highest = get_card_value(*card);
            suite = get_card_suite(*card);
        }
    }
    return suite;
}

//return the first instance of the amount repeated
fn getValueOfRepeat(hand:[i32;5],repeatAmount:i32)->i32{
    for card in &hand{
        if doesValRepeatExactly(hand,repeatAmount, get_card_value(*card) ) {
            return get_card_value(*card);
        }
    }
    println!("Error on getValueOfRepeat - returned -1");
    return -1;//this is an error state
}

fn doesValRepeatExactly(hand:[i32;5],repeatAmount:i32,faceVal:i32)->bool{
    let mut rep = 0;
    for card in &hand {
        if get_card_value(*card) == faceVal {
            rep = rep +1;
        }
    }
    return ( rep == repeatAmount);
}


fn sortHandOnValue(hand:[i32;5])->[i32;5]{
    let mut returnHand = hand.clone();
    let mut indexCheck = 0;
    while indexCheck<4 {
        if ( get_card_value(returnHand[indexCheck]) > get_card_value(returnHand[indexCheck+1]) ){
            let temp = returnHand[indexCheck];
            returnHand[indexCheck] = returnHand[indexCheck+1];
            returnHand[indexCheck+1] = temp;
            indexCheck = 0;
        }else{
            indexCheck = indexCheck+1;
        }
    }
    return returnHand;
}

fn getSuitOfFaceVal(hand:[i32;5],faceVal:i32)->i32{
    for card in &hand{
        if get_card_value(*card) == faceVal{
            return get_card_suite(*card);
        }
    }
    println!("ERROR: Could not find {:?} in getSuitOfFaceVal", faceVal);
    return -1;
}

fn max(a:i32,b:i32)->i32{
    if( a>b){
        return a;
    }
    return b;
}