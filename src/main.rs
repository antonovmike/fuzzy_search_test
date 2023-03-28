use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // let text_file = "dbo.GOOD.Table.sql";
    // let file = File::open(text_file).unwrap();
    // let reader = BufReader::new(file);

    // for (index, line) in reader.lines().enumerate() {
    //     if index > 60 && index < 64 {
    //         let line = line.unwrap();
    //         println!("{}\n{}", index + 1, line);
    //     }
    // }

    let long_string = "INSERT [dbo].[GOOD] ([ID], [NAME], [PRNAME], [EDIZM], [PRODUCER], [NMN], [GOOD], [GROUPS], [GROUPS_CH], [GRKOL], [MANDATORY], [BARCODE], [MX], [REC], [STATE], [DATACHANGE], [ANATACIA], [SUM_GOOD], [FOM_GOOD], [TRADENAME], [DOSAGE], [FORM], [REFERENCE], [ART], [SENDSPR], [FOM_SUMMARY_GROUPS], [MED], [CLASSCODE], [ATXID], [MNNFR], [Label], [UnitCode], [WHTYPE], [TESTDATE], [CLASSCODEOLD]) VALUES (313911, N'\" # \" !130', N'.$ , !(', N'', 55556, 3685, NULL, 40, 185, 130, NULL, N'3629002748139', N'', 0, 3, CAST(N'2023-02-09T09:01:07.057' AS DateTime), N'', NULL, 6924, N'', N'', N'', NULL, 0, 1, NULL, 0, N'03004066004025012', 588, 1003685, 0, N'1146307', 1, CAST(N'2023-02-09T09:01:06.380' AS DateTime), NULL)";
    let shorter_string = &long_string[398..];
    let parts = shorter_string.split(", N'");
    for part in parts {
        println!("{}", part)
    }
}
