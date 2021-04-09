use std::io::Write;

mod models;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() -> anyhow::Result<()> {
    let mut cards: Vec<models::Cards> =
        serde_json::from_reader(std::fs::File::open("cards/cards.json")?)?;

    for i in &mut cards {
        for j in &mut i.white {
            j.text = strip_markdown::strip_markdown(&j.text);
        }
        for j in &mut i.black {
            j.text = strip_markdown::strip_markdown(&j.text);
        }
    }

    let file = std::fs::File::create("cards.bincode.br")?;

    let mut compressor = brotli2::write::BrotliEncoder::new(file, 11);
    compressor.write_all(bincode::serialize(&cards)?.as_slice())?;
    compressor.finish()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    #[test]
    fn decomp_bincode() -> anyhow::Result<()> {
        let file = std::fs::read("cards.bincode.br")?;
        let fileout = std::fs::File::create("cards.bincode")?;
        let mut decomp = brotli2::write::BrotliDecoder::new(fileout);
        decomp.write_all(file.as_slice())?;
        Ok(())
    }
}
