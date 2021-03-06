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
    let binLength: i32 = listBin[0].len().to_string().parse::<i32>().unwrap();
    let mut testList: Vec<String> = Vec::new();
    testList.push("2d".to_string());

    let (gRate, aRate, commonFalseList, commonTrueList) =
        get_common_bit(&listBin, commonFalseList, commonTrueList);

    let gRateString: String = gRate.into_iter().map(|i| i.to_string()).collect::<String>();
    let aRateString: String = aRate.into_iter().map(|i| i.to_string()).collect::<String>();

    let gCon = isize::from_str_radix(&gRateString, 2).unwrap();
    let aCon = isize::from_str_radix(&aRateString, 2).unwrap();

    //println!("{} * {} = {}", gCon, aCon, gCon * aCon);
    let o2RateBI: String = get_part_2(&listBin, binLength);
    let cO2RateBI: String = get_part_2_Co2(&listBin, binLength);
    //println!("oxygen = {}, Co2 = {}", o2RateBI, cO2RateBI);

    let o2Rate = isize::from_str_radix(&o2RateBI, 2).unwrap();
    let cO2Rate = isize::from_str_radix(&cO2RateBI, 2).unwrap();
    println!("{} * {} = {}", o2Rate, cO2Rate, o2Rate * cO2Rate);
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

pub fn get_common_bit<'a>(
    binList: &'a Vec<String>,
    mut falseList: Vec<u32>,
    mut trueList: Vec<u32>,
) -> (Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>) {
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

    (gamaRate, epsilonRate, falseList, trueList)
}

pub fn get_part_2<'a>(binList: &'a Vec<String>, mut lCount: i32) -> String {
    let iter = binList.iter();
    let oLCount = binList[0].len().to_string().parse::<i32>().unwrap(); // original length
    let mut pos = oLCount - lCount;
    let mut tCount: i32 = 0;
    let mut fCount: i32 = 0;
    lCount -= 1;
    let mut newList: Vec<String> = Vec::new();

    if pos != binList[0].len().to_string().parse::<i32>().unwrap() {
        for val in iter {
            if val.chars().nth(pos as usize).unwrap() == '1' {
                tCount += 1;
            } else {
                fCount += 1;
            }
        }
        let iter = binList.iter();

        if tCount >= fCount {
            for val in iter {
                if val.chars().nth(pos as usize).unwrap() == '1' {
                    newList.push(val.to_string());
                }
            }
        } else {
            for val in iter {
                if val.chars().nth(pos as usize).unwrap() == '0' {
                    newList.push(val.to_string());
                }
            }
        }

        //println!("{:?}", newList);

        //binList[0].to_string()

        if newList.len() == 1 {
            newList[0].to_string()
        } else {
            let ans: String = get_part_2(&newList.clone(), lCount);
            ans
        }

        // "dome".to_string()
    } else {
        println!("{:?}", binList);
        let some: String = binList[0].to_string();
        some
    }
}

pub fn get_part_2_Co2<'a>(binList: &'a Vec<String>, mut lCount: i32) -> String {
    let iter = binList.iter();
    let oLCount = binList[0].len().to_string().parse::<i32>().unwrap(); // original length
    let mut pos = oLCount - lCount;
    let mut tCount: i32 = 0;
    let mut fCount: i32 = 0;
    lCount -= 1;
    let mut newList: Vec<String> = Vec::new();

    if pos != binList[0].len().to_string().parse::<i32>().unwrap() {
        for val in iter {
            if val.chars().nth(pos as usize).unwrap() == '1' {
                tCount += 1;
            } else {
                fCount += 1;
            }
        }
        let iter = binList.iter();

        if tCount >= fCount {
            for val in iter {
                if val.chars().nth(pos as usize).unwrap() == '0' {
                    newList.push(val.to_string());
                }
            }
        } else {
            for val in iter {
                if val.chars().nth(pos as usize).unwrap() == '1' {
                    newList.push(val.to_string());
                }
            }
        }

        //println!("{:?}", newList);

        //binList[0].to_string()

        if newList.len() == 1 {
            newList[0].to_string()
        } else {
            let ans: String = get_part_2_Co2(&newList.clone(), lCount);
            ans
        }

        // "dome".to_string()
    } else {
        println!("{:?}", binList);
        let some: String = binList[0].to_string();
        some
    }
}
