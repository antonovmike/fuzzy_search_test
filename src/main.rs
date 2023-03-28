use std::fs::File;
use std::io::{BufRead, BufReader};
use utf16_reader::read_to_string;

fn main() {
    let text_file = "dbo.GOOD.Table.sql";
    let file = File::open(text_file).unwrap();
    // let reader = BufReader::new(file);

    // let mut lines: Vec<String> = vec![];

    // for (index, line) in reader.lines().enumerate() {
    //     if index > 60 && index < 64 {
    //         let sliced = &line.unwrap()[..];
    //         lines.push(sliced.to_string())
    //     }
    // }

    // for i in lines {
    //     let shorter_string = &i[816..];
    //     let parts = shorter_string.split("N");
    //     let mut index = 0;
    //     for part in parts {
    //         if index == 0 || index == 1 || index == 2 {
    //             println!("{}\n{}", index, part)
    //         }
    //         index += 1
    //     }
    // }

    let r = BufReader::new(file);
    let s = utf16_reader::read_to_string(r);
    println!("{}", s);

    // let long_string = "INSERT [dbo].[GOOD] ([ID], [NAME], [PRNAME], [EDIZM], [PRODUCER], [NMN], [GOOD], [GROUPS], [GROUPS_CH], [GRKOL], [MANDATORY], [BARCODE], [MX], [REC], [STATE], [DATACHANGE], [ANATACIA], [SUM_GOOD], [FOM_GOOD], [TRADENAME], [DOSAGE], [FORM], [REFERENCE], [ART], [SENDSPR], [FOM_SUMMARY_GROUPS], [MED], [CLASSCODE], [ATXID], [MNNFR], [Label], [UnitCode], [WHTYPE], [TESTDATE], [CLASSCODEOLD]) VALUES (313911, N'\" # \" !130', N'.$ , !(', N'', 55556, 3685, NULL, 40, 185, 130, NULL, N'3629002748139', N'', 0, 3, CAST(N'2023-02-09T09:01:07.057' AS DateTime), N'', NULL, 6924, N'', N'', N'', NULL, 0, 1, NULL, 0, N'03004066004025012', 588, 1003685, 0, N'1146307', 1, CAST(N'2023-02-09T09:01:06.380' AS DateTime), NULL)";
    // let mut v: Vec<u16> = long_string.encode_utf16().collect();
    // v.push(0);
    // let a = String::from_utf16(&v);
    // println!("{:?}", a);

    // let shorter_string = &long_string[398..];
    // let parts = shorter_string.split(", N'");

    // let mut index = 0;
    // for part in parts {
    //     if index == 1 || index == 2 {
    //         println!("{}", part)
    //     }
    //     index += 1
    // }
}
