fn main(){
    // println!("hello world");
    // let mut deck:[i32; 10]=[1,2,3,4,5,6,7,8,9,10]; //sample deck
    // //println!("{}", check_sequence_helper([1,2,3,4,6]));
    // return_card_frequency_helper([1,3,3,3,3],3);
    // println!("here {}",check_flush([1,2,3,4,5]));
    // println!("here2 {}",check_three_of_a_kind([1,2,2,2,2]));

    println!("here {:?}",get_high_card([1,2,3,4,5]));
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
    if reduced_hand==[0,1,2,3,12]{ //the case when ace is low(straight)
        return true;
    }
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
        return vec==vec![1,1,1,2,2]||vec==vec![2,2,3,3,3];
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
    println!("{:?} converted hand",converted_hand);
    if check_straight(hand){
        println!("in straight");
        return converted_hand[3];
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
    determineTieBreaker(hand1,hand2,rank1);


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












//--------------------------------- tie breaker functions---------------------------

fn determineTieBreaker(hand1:[i32;5],hand2:[i32;5],rank:i32)->[i32;5]{
    if rank==10 {
        return tieBreakRoyalFlush(hand1,hand2);//
    }
    /*
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
    */

    //temp
    return tieBreakRoyalFlush(hand1,hand2);
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
    let kicker2 = getValueOfRepeat(hand2,2);
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