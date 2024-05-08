use noiserand::NoiseRand;
use rand_core::RngCore;
use std::io::{stdin, stdout, Write};

type DataItem = (&'static str, &'static str, &'static str);

const COUNTRIES: usize = 34;
#[rustfmt::skip]
static mut MAY_2024: [DataItem; COUNTRIES] = [
    ("Antigua and Barbuda", "🇦🇬", "St. John's"),
    ("Argentina", "🇦🇷", "Buenos Aires"),
    ("Bahamas", "🇧🇸", "Nassau"),
    ("Barbados", "🇧🇧", "Bridgetown"),
    ("Belize", "🇧🇿", "Belmopan"),
    ("Bolivia", "🇧🇴", "Sucre"),
    ("Brazil", "🇧🇷", "Brasilia"),
    ("Canada", "🇨🇦", "Ottawa"),
    ("Chile", "🇨🇱", "Santiago"),
    ("Colombia", "🇨🇴", "Bogota"),
    ("Costa Rica", "🇨🇷", "San Jose"),    
    ("Dominica", "🇩🇲", "Roseau"),
    ("Dominican Republic", "🇩🇴", "Santo Domingo"),
    ("Ecuador", "🇪🇨", "Quito"),
    ("El Salvador", "🇸🇻", "San Salvador"),
    ("Grenada", "🇬🇩", "St. George's"),
    ("Guatemala", "🇬🇹", "Guatemala City"),
    ("Guyana", "🇬🇾", "Georgetown"),
    ("Haiti", "🇭🇹", "Port-au-Prince"),
    ("Honduras", "🇭🇳", "Tegucigalpa"),
    ("Jamaica", "🇯🇲", "Kingston"),
    ("Mexico", "🇲🇽", "Mexico City"),
    ("Nicaragua", "🇳🇮", "Managua"),
    ("Panama", "🇵🇦", "Panama City"),
    ("Paraguay", "🇵🇾", "Asuncion"),
    ("Peru", "🇵🇪", "Lima"),
    ("Saint Kitts and Nevis", "🇰🇳", "Basseterre"),
    ("Saint Lucia", "🇱🇨", "Castries"),
    ("Saint Vincent and the Grenadines", "🇻🇨", "Kingstown"),
    ("Suriname", "🇸🇷", "Paramaribo"),
    ("Trinidad and Tobago", "🇹🇹", "Port of Spain"),
    ("United States", "🇺🇸", "Washington, D.C."),
    ("Uruguay", "🇺🇾", "Montevideo"),
    ("Venezuela", "🇻🇪", "Caracas"),    
];

#[derive(PartialEq, Copy, Clone)]
enum HintAmount {
    None,
    Full,
    Partial,
}

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let args: &[&str] = &args.iter().map(|x| x.as_str()).collect::<Vec<&str>>();

    let mut stdout = stdout();

    if args.contains(&"--help") {
        #[rustfmt::skip]writeln!(&mut stdout, "\n--- HELP ---\n")?;
        #[rustfmt::skip]writeln!(&mut stdout, "    --help         |this help")?;
        #[rustfmt::skip]writeln!(&mut stdout, "    --nocolor      |no output colorization")?;
        #[rustfmt::skip]writeln!(&mut stdout, "    --list         |outputs country list")?;
        #[rustfmt::skip]writeln!(&mut stdout, "    --flag-only    |flag only mode")?;
        #[rustfmt::skip]writeln!(&mut stdout, "    --country-mode |guessing countries instead of capitals")?;
        #[rustfmt::skip]writeln!(&mut stdout, "    --hint:none    |provides no hint on error")?;
        #[rustfmt::skip]writeln!(&mut stdout, "    --hint:partial |only country or capitals hint, use with --flag-only\n")?;

        stdout.flush()?;
        return Ok(());
    }

    if args.contains(&"--list") {
        writeln!(&mut stdout, "┍{:─<35}┯────┯{:─<28}┑", "", "")?;
        writeln!(&mut stdout, "┆{:^35}┆Flag┆{:^28}┆", "Country", "Capital")?;
        writeln!(&mut stdout, "┠{:─<35}┼┄┄┄┄╉{:─<28}┨", "", "")?;

        let mut inx = 0;
        unsafe {
            loop {
                let it = MAY_2024[inx];

                writeln!(&mut stdout, "┆{:<35}│{:^4}│ {:<26} ┆", it.0, it.1, it.2)?;
                inx += 1;

                if inx < COUNTRIES {
                    writeln!(&mut stdout, "┝{:┄<35}┼────╉{:┄<28}┫", "", "")?;
                } else {
                    break;
                }
            }
        }
        writeln!(&mut stdout, "┕{:─<35}┷────┻{:─<28}┙", "", "")?;
        stdout.flush()?;
        return Ok(());
    }

    let mut colorize = true;
    let mut flag_mode = false;
    let mut hint_amount = HintAmount::Full;
    let mut country_mode = false;

    if args.contains(&"--flag-only") {
        flag_mode = true;
    }

    if args.contains(&"--nocolor") {
        colorize = false;
    }

    if args.contains(&"--hint:none") {
        hint_amount = HintAmount::None
    } else if args.contains(&"--hint:partial") {
        if flag_mode {
            hint_amount = HintAmount::Partial
        }
    }

    if args.contains(&"--country-mode") {
        country_mode = true;
    }

    writeln!(
        &mut stdout,
        "\n----> Welcome to the Americas Capitals Game <----\n"
    )?;
    writeln!(
        &mut stdout,
        "Acquiring quantum fluctuations based seed. Check https://qrng.anu.edu.au/ for more.\n\n"
    )?;
    stdout.flush()?;

    let mut nr = NoiseRand::new();
    let rn = nr.next_u32();
    let b0 = rn.to_ne_bytes()[0];

    const LT_INX: usize = COUNTRIES - 1;

    let mut ix1 = 0;
    let mut ix2 = match b0 as usize {
        x if x > LT_INX => x % COUNTRIES,
        x => x,
    };

    while ix2 < COUNTRIES {
        unsafe {
            let swap = MAY_2024[ix2];
            MAY_2024[ix2] = MAY_2024[ix1];
            MAY_2024[ix1] = swap;
        }
        ix1 += 1;
        ix2 += 1;
    }

    for i in 0..LT_INX {
        ix1 = i;
        ix2 = LT_INX;
        while ix1 < ix2 {
            unsafe {
                let swap = MAY_2024[ix2];
                MAY_2024[ix2] = MAY_2024[ix1];
                MAY_2024[ix1] = swap;
            }

            ix1 += 1;
            ix2 -= 1;
        }
    }

    let mut buff = String::with_capacity(2);
    let num = loop {
        write!(&mut stdout, "Tell batch size [1-{}]: ", COUNTRIES)?;
        stdout.flush()?;
        buff.clear();

        read_line(&mut buff);
        let try_num = buff.parse::<u8>();

        if let Ok(num) = try_num {
            let num = num as usize;
            if num > 0 && num < COUNTRIES + 1 {
                break num;
            }
        }

        writeln!(
            &mut stdout,
            "   ┖━━━━╾ Error ━╾ {}",
            colorized(colorize, &buff, "\x1b[0;43m")
        )?;
    };

    let (mut print, mut color);

    let mut inx = 0;
    while inx < num {
        let (country, flag, capital) = unsafe { MAY_2024[inx] };

        let mut question = String::new();

        if country_mode {
            question.push_str("Country for ");
        } else {
            question.push_str("Capital of ");
        }

        if !flag_mode {
            question.push_str(if country_mode { capital } else { country });

            question.push_str(", ");
        }

        question.push_str(&flag);

        writeln!(&mut stdout, "{}?", question)?;
        stdout.flush()?;

        buff.clear();
        read_line(&mut buff);

        let mut correct_answer = false;
        let buff = &buff;
        let answer = if country_mode { country } else { capital };

        if answer == buff {
            correct_answer = true;
        }

        let mut hint = String::from("");
        if correct_answer {
            inx += 1;
            (print, color) = ("Yes", "\x1b[0;32m");
        } else {
            match hint_amount {
                HintAmount::Full => {
                    hint.push_str(", ");

                    hint.push_str(if country_mode { country } else { capital });

                    if flag_mode {
                        hint.push_str(" (");
                        hint.push_str(if country_mode { capital } else { country });
                        hint.push_str(")");
                    }
                }
                HintAmount::Partial => {
                    hint.push_str(". Hint: ");
                    hint.push_str(if country_mode { capital } else { country });
                }
                HintAmount::None => {}
            }

            (print, color) = ("Never", "\x1b[0;31m");
        }

        writeln!(
            &mut stdout,
            "{}{}.\n",
            colorized(colorize, print, color),
            hint
        )?;
    }

    stdout.flush()?;
    Ok(())
}

fn read_line<'a>(buff: &'a mut String) -> &'a str {
    while let Err(_) = stdin().read_line(buff) {}
    *buff = buff.replace("\n", "").replace("\r", "");
    buff.trim_start().trim_end()
}

fn colorized(colorize: bool, txt: &str, color: &str) -> String {
    let mut txt = txt.to_string();
    if colorize {
        txt.insert_str(0, color);
        txt.push_str("\x1b[0;0m");
    }

    txt
}
