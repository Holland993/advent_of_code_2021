use std::fs;

fn main() {
    let filename = "../input.txt";
    let content = fs::read_to_string(filename).expect("Dude this is no file i want");

    let mut commonTrueList: Vec<u32> = Vec::new();
    let mut commonFalseList: Vec<u32> = Vec::new();
    let mut eRate: Vec<u32> = Vec::new();
    let mut gRate: Vec<u32> = Vec::new();

    let (listBin, commonFalseList, commonTrueList) =
        Loop_through(&content, commonFalseList, commonTrueList);
    let binLength: u32 = listBin[0].len().to_string().parse::<u32>().unwrap();

    let (gRate, aRate) = get_common_bit(listBin, commonFalseList, commonTrueList);

    let gRateString: String = gRate.into_iter().map(|i| i.to_string()).collect::<String>();
    let aRateString: String = aRate.into_iter().map(|i| i.to_string()).collect::<String>();

    let gCon = isize::from_str_radix(&gRateString, 2).unwrap();
    let aCon = isize::from_str_radix(&aRateString, 2).unwrap();

    println!("{} * {} = {}", gCon, aCon, gCon * aCon);
}

pub fn Loop_through<'a>(
    content: &'a str,
    mut fList: Vec<u32>,
    mut tList: Vec<u32>,
) -> (Vec<String>, Vec<u32>, Vec<u32>) {
    let mut binList: Vec<String> = Vec::new();
    for line in content.lines() {
        binList.push(line.to_string());
    }

    let iter: u32 = binList[0]
        .chars()
        .count()
        .to_string()
        .parse::<u32>()
        .unwrap();

    for i in 0..iter {
        fList.push(0);
        tList.push(0);
    }

    (binList, fList, tList)
}

pub fn get_common_bit(
    binList: Vec<String>,
    mut falseList: Vec<u32>,
    mut trueList: Vec<u32>,
) -> (Vec<u32>, Vec<u32>) {
    let mut gamaRate: Vec<u32> = Vec::new();
    let mut epsilonRate: Vec<u32> = Vec::new();
    let iter = binList.iter();

    for val in iter {
        for i in 0..val.chars().count().to_string().parse::<u32>().unwrap() {
            if val.chars().nth(i.try_into().expect("d")).expect("no") == '1' {
                falseList[i as usize] += 1;

            } else if val.chars().nth(i.try_into().expect("d")).expect("no") == '0' {
                trueList[i as usize] += 1;
                
            }
        }
    }
    let gamaIter = trueList.len();
    for i in 0..gamaIter {
        if falseList[i as usize] > trueList[i as usize] {
            gamaRate.push(1);
        } else if falseList[i as usize] < trueList[i as usize] {
            gamaRate.push(0);
        }
    }

    let epsilonIter = gamaRate.iter();
    for val in epsilonIter {
        if val.clone() == 1 {
            epsilonRate.push(0);
        } else if val.clone() == 0 {
            epsilonRate.push(1);
        }
    }

    (gamaRate, epsilonRate)
}

// Take a list of binary digits of size column by column and find the most common bit in that column
// e.g. 00100, 11110, 10110, 10111, 10101, 01111, 00111, 11100, 10000, 11001, 00010, 01010
// e.g. making separate counters gama rate
// would end up 10110
// Then find the inverted bits 01001
// convert binary to Decimal and multiply both numbers to get power consumption
// e.g. 01001 => 9, 10110 => 22 (Therefore 22 * 9 = 198)
