use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

pub fn get(year: i16, day: i8) -> String {
    let cached = get_cached_input(year, day);
    return match cached {
        Ok(input) => {
            // Check if cached content is an error message
            if input.contains("Puzzle inputs differ by user")
                || input.contains("Please log in")
                || input.contains("Please don't repeatedly request this endpoint before it unlocks")
            {
                eprintln!("Cached file contains error response, deleting...");
                let _ = fs::remove_file(get_cached_file_path(year, day));
                // Trigger re-download
                return get(year, day);
            }
            input
        }
        Err(_) => {
            println!("could not find cached file, downloading...");
            let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
            let cookie = get_sessionid();
            return match get_content(&url, &cookie) {
                Ok(input) => {
                    cache_input(&input, year, day);
                    input
                }
                Err(_) => {
                    // Delete bad session cookie and prompt for new one
                    let _ = fs::remove_file(get_sessionid_file_path());
                    save_new_sessionid();
                    return get(year, day);
                }
            };
        }
    };
}

fn cache_input(input: &String, year: i16, day: i8) {
    let file_path = get_cached_file_path(year, day);
    println!("caching input to {}", file_path.to_string_lossy());
    fs::create_dir_all(get_cache_year_path(year)).expect("could not create directories");
    let file = File::create(file_path.clone());
    match file {
        Ok(mut f) => {
            f.write_all(input.as_bytes())
                .expect("could not write to file");
        }
        Err(e) => {
            eprintln!("encountered an error: {}", e);
        }
    };
}

fn get_cache_dir() -> PathBuf {
    dirs::home_dir()
        .expect("could not get home dir")
        .join(".cache/aoc/")
}

fn get_inputs_dir() -> PathBuf {
    get_cache_dir().join("inputs")
}

fn get_sessionid_file_path() -> PathBuf {
    get_cache_dir().join("sessionid")
}

fn save_new_sessionid() -> String {
    let path = get_sessionid_file_path();
    eprintln!("\n=== Advent of Code Session Cookie Required ===");
    eprintln!("To download puzzle inputs, you need your session cookie from adventofcode.com");
    eprintln!("\nHow to get it:");
    eprintln!("  1. Go to https://adventofcode.com and log in");
    eprintln!("  2. Open browser DevTools (F12)");
    eprintln!("  3. Go to Application/Storage tab > Cookies");
    eprintln!("  4. Find the 'session' cookie");
    eprintln!("  5. Copy the entire cookie value (format: session=abc123...)");
    eprintln!("\nAlternatively, copy the Cookie header value from Network tab");
    eprintln!("(format: session=abc123...)");
    eprintln!("===============================================\n");

    print!("Paste your session cookie here: ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    let trimmed = line.trim();

    fs::create_dir_all(get_cache_dir()).unwrap();
    fs::write(&path, trimmed).expect("unable to write session cookie to file");
    eprintln!("✓ Session cookie saved to {}", path.display());
    return trimmed.to_string();
}

fn get_sessionid() -> String {
    let path = get_sessionid_file_path();
    let id = fs::read_to_string(&path);
    if id.is_err() {
        return save_new_sessionid();
    }
    id.unwrap().trim().to_owned()
}

fn get_content(url: &String, cookie: &String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    println!("url:    '{}'", url);
    println!("cookie: '{}'", cookie);
    let res = client.get(url).header("Cookie", cookie.as_str()).send()?;

    // Check if response indicates authentication failure
    let text = res.text()?;
    if text.contains("Puzzle inputs differ by user") || text.contains("Please log in") {
        eprintln!(
            "\nAuthentication required: {}",
            text.lines().next().unwrap_or(&text)
        );
        return Err("Authentication failed - invalid or missing session cookie".into());
    }

    Ok(text)
}

fn get_cache_year_path(year: i16) -> PathBuf {
    get_inputs_dir().join(year.to_string())
}

fn get_cached_file_path(year: i16, day: i8) -> PathBuf {
    get_cache_year_path(year).join(day.to_string())
}

fn get_cached_input(year: i16, day: i8) -> Result<String, io::Error> {
    let file_path = get_cached_file_path(year, day);
    fs::read_to_string(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading_cached_file_works() {
        let result = get_cached_input(2022, 6);
        assert!(result.is_ok())
    }

    #[test]
    fn reading_session_works() {
        let result = get_sessionid();
        println!("got sessionid: {}", result);
        assert!(result.len() > 0);
    }

    #[test]
    fn getting_input_works() {
        let result = get(2022, 6);
        assert_eq!(result, "lrdrgrcggjrgrddfqfqrqppptrtssnqqqlzqqslspppsddfnnwgnnwhhjttnsnswnnbqbrrcqrqdqcdddlnddwwlmmlmtmwmggjgjmmmrppqhhswhwhmhchtchhsttvbtvvpspjjqrrgqrqmqvmvzmvzzrnnqlnlwlttjstjtjvjddrzzrnrmrbmbwwzppnqpqmmjnnvqnvvdzdpdvvmhvhhmshmsssvnnqppclcqlcqctqtfffvrfvrffptfftppncncwnntfntftdftfhfzzfwzzcddngdndccrttjmmcddrvrwwnntggghccjbjvbvjvlvjlvlgljlnjjtvjjfddmwwhnnqnrnccwllpmprmmspsvsrsslstsllpmllfzlflqltlssdbbdsdqsdqqgfgfjjtzjzmmsnsmnnzssgfgzgjgwwqfqtfthhhczzpwprwwqttqcqctcpcrrvwrvrggfvfqvvbqqhnnczcjjjfbfhbhjhssnwwfswszwwgbgzgczcbbvmvrrdgrddfwdffvsvtstztftfvfssgffnrnnzzqddltdtbdttnnfrnnfrfddpmpqqplqqbdbzbnnnrttjfttprtrllqhhwjhjghjjcrcqqznztzllldjjhqqspsfpfprrsfspsrrzbbjmjssnjnsjnjllbttgrtrnnqhhvjhvvrrplrprtrtprtrdtdfttwdttqtttwvtvtmtntnnfvfsfnndsnnnfqfmmcdmdppsmpmnnwhhlwwrvwrwvwlwnwhwjwvwbvwvnnqtnnttcgtccbggzgwzwczcpzcpzpzgzngnzzhfhdhzzzsggtssddgtdtndtdllpbpjpgjjsllvqvmmvjmvjvbbfzzbdzzghhzjzvvnffwsshqhqhjhmmtgmgpmgmpmzzzhtzzbzsbzsbzszvsvgsvspvsstwtgwgqgjggsfsmffhlhmhdmhmgmrmffjcjggqgsghgttjffprpmmscsggswssgzzsqzqjqwqjwqwswjjqpjpllwrwffvfbfpbffjsswgsshjsjqssqrrvcrrnvvdnnwndnffpdffjlffrjrlrqrmmlzlttpccgchggdjjhwwvqvvrprdrttlvvtlvtlvvvhqhwhmmpmrrjsrrzbzfzdzhdzzwgwwqllmmthmmvpprqprrzmrrfrmfrrmfmjmwwhtwhttnbbhbnnhbnnmhmgmjgmgfmmhpplwplwlswwjjbpbhpbbzmmgjmmggbzbssppjtttfmfwwcwmmbmrmbrblbmmszzvgvzgvgvdggjgsjgsjgghfgfngffsnfssdpdgpgrgnrggrlggdwgdgddfwddjdrrppnnqbnntmmrmlmrrgvvqjjwzzgnzzdtzzbccnbcbttbzzcqqrcqqpspbphbphhnpnccggczcgcpgcpctppqwpwccsgsttgtbttvftfcfbfvfppfpnpffmjmljmmrtrsrttnhhcvctclttrffhzhrzzdpzzwvwddcggpbptbpbzpzdpzddbjjdvvwmmfvfcfmfmmvrmvrmmcdmdqqhjqhqmmstmmdnnvssmbsmmnhmnmlmttlthtvhvvgbvgvbvqbvbvnbvvpwwhwsszpszppjljsllzhzcchmhdhphvvtpvvfssvmvcmmrjmmgfmfwmfmsspbbpnnsbsgbbzzllzzsqqswqqsbqqlvqlqzzbrzzqsqdqjqzqlqccqbqlblpprrthrttjzzfvzzpvpbppqjpjjtllmrmzzwnnrggpfpllwhhnmnpmmdppwnnwpnnzlzggvpvgpprdrzddlmddjndjjrljlsjshhqbbhhmhvhqhzqqlbqlqnqqmhmwmhwwgbwbpbhpbblnnshnshslllrcrncctnthtllmglmlmnmfnfllrbrmdqsncjfbjnghmvpctfzttqwsfptwhfhmdhzgbmrpqvsbbwdwdnqgclrgvdbqvtnzvlzmdjzgbhbtqjhrgqqjgwcvpqhhwsmmflgsjsvbnjcjcqwqqfcbvfrllbpphfglptjfczmnfnrmqdgrhbvrddwvhnbffsnzntvldrwmgqmqdbmmpdjhpdqfsbzsfbtdzlfjwtwfpfrhzcwwrcfpdjzjzqvcnhvdvlbdbjfrnmchdjprbfpzdtmjhdjtrwfrsghngbswhfzdfvhdmnqwnpspdjgsfcjjhrlnpncfdhzfzglgvrjlzdcncbfjhpnrvwnqbzhtdfcnpnpddsnjcbgdvzlsmpnlbftgjfsjsslljppjhtjfddmwbtrvmhvshnvhhfdsqhpmgzfrcqtpdmfcwglnbjzmtmzshzsrnbzlwlzdjzzsqfmjzbnprjdzswwcgjjwslhnfhbflvcpdfzwhzzjmfvpfhzbrcdvtlmbvrqvvtlpbdnshtsvlcnlwqzcnfhzndfjbhgwpvspdvhfptnfvwznscmgthspwpjfwlbbcrzgthrhnvscvfvwrwvtzpjqmwdbjjslvzlmqstzwqtsdsqqjpqthgnshpjncwgvppngvvfrjpztftgjbbmgdddrcgjggwpfrvfqbnzvwtscftbwfssmrgrbchjmvqcdgbdmmsswvplnvnjrnqzhttbzhlthzjslgtgrnqpghtbjnbftpdttvqhrljhnfqllrhfmzcnnwrljmchsvdbgvwmrpfzjsbngpffpgntrgflntbjnjwqbdhvhssvptdvvqvtrgqhhzrcfnnmbtzqrcghggcqhndggqzzgswflhqqrgmnggbrvpwqgtlptgmzdvgztgfqdzwnbrvvqhpnmcmptqljmqqmsslfcmgpnmnjmrsngrtbsffqbwlhsrwbzdcqvpbptpjphcjnwsmrjdbbrwftsnrgfhpjvsjcwpmpvfjtjvfnnwdjdttsjrqrgsznzqwjmsvscdtpptmdhqvwngtldtsnstjmwwrwwzdzvtndrrhgsgshzlpdrmlsgvplvbfrffvgvhmncbjdlqspbpdpcdsffrjzptltsznwwqbvnrwfdtrlbbvsmzjrhhvscqfwsqtmwnpjfgnjcttwphtqqhnvgjmzvczcrlmjjgwgnprcmmprltfrgrjlhvggdwvpnrqrtfwhpsfnjfvmvgzpmhrqmggldmsvztwcwzgblfbvbfvnshrdppzcvjwbjmsncwnbnjwbmwqjjwtjmwzpdvpwrfdtrqhnltgntvglgspvnbcsvnpppwmgphhsdqtpmrzmbdqlghsgjnnbhflzwghzzhdfsvjjcjfcssmvrmfqbgzcfwmhpvjqqrrhpsffczwgbjgwqlvzrvnhvzrqnfllqtrcjhmpdjfpqnlzhdnsctbfhsbpgwqdjdjhfbqzvzpsgmqwfjsffdjcqfmjgzqzvfsbhhvnwlfjfmdnphggdbnmpznlrzbnlbvhvplhjbzdmspnnlbctgzphghpngppdpdfbcdcpfntqlrwclsvnpljdbwcbhwzzdhbhsmslvvtjsmqmtpnhmmqnhfggnlpfthdhpwmrhzgfpwsffnrdqszcttrjsqzjqgspltfzzjnzwdfzvnmncwnphmjvcwlgrzcvpzcbndvhtjnfsgjtjdqfmgcgtgppsrcqwdjcqfddlhhnlfjrnsnssqblmnjjvmfbghtwwgcpgzfddvzsrsghqfrpvdtqmjfrzpvclmsnpmrqngdwvznlhdpbrzqtswfhnblnnbwjcwwbwfrgcdgrpphnslgnwbtfcgcmvvllwgvrcrdfcfwvwmdhhzmmnzmjgmgrgpdhngjgmhlqwtgzlngrbbfwfjrpwbqjnvdggrcfdgphfctvbmjnwfbpwrvbdbjrbhtnhfvhwvrptptsdrnzmnlwgbrwcswrccwdftgvjnvhffghdvhltjwhppfwfnmmtwclzzftzwvmhpmgvdsqzrfwqsmcgswnzjcnrvdgjlqdjrczphsvldlfzwdwmpncpvgqsvgpfpsfbgbmdhnfqbhdqwwwfdgqtmjlfbztsrwjrtqnrfpfqgplznpftrnjzhzcrnngqpwbrpbhlbfsgrpwfflrpbqdrqdplgcn
");
    }
}
