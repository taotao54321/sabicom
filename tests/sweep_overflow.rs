use meru_interface::EmulatorCore as _;
use sabicom::{Config, Nes};

/// 矩形波 A のスイープ設定が「negate 有効、シフト回数 0」のとき整数オーバーフローが発生する。
#[test]
fn test_sweep_overflow() -> anyhow::Result<()> {
    let rom = make_rom();

    let mut nes = Nes::try_from_file(&rom, None, &Config::default())?;
    nes.exec_frame(false);

    Ok(())
}

fn make_rom() -> Vec<u8> {
    let mut header = vec![
        b'N', b'E', b'S', b'\x1A', // iNES magic
        2,       // 16KiB PRG 個数
        1,       // 8KiB CHR 個数
        0, 0, // マッパー 0
    ];
    header.resize(16, 0);

    // 矩形波 A スイープを「negate 有効、シフト回数 0」に設定して無限ループするだけ。
    let mut prg = vec![
        0xA9, 0x08, // lda #$08
        0x8D, 0x01, 0x40, // sta $4001
        0xD0, 0xFE, // : bne :-
    ];
    prg.resize(0x8000, 0);
    prg[0x7FFC..][..2].copy_from_slice(&0x8000_u16.to_le_bytes());

    let chr = vec![0; 0x2000];

    [header, prg, chr].concat()
}
