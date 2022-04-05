use ffmpeg_next::{codec, decoder};

pub fn open_decoder(stream: &ffmpeg_next::Stream) -> Result<decoder::Video, ffmpeg_next::Error> {
    let ctx = codec::Context::from_parameters(stream.parameters())?;
    match overwrite_decoder(ctx.id()) {
        Some(codec) => ctx.decoder().open_as(codec).and_then(|o| o.video()),
        None => ctx.decoder().video(),
    }
}

fn overwrite_decoder(id: codec::Id) -> Option<codec::codec::Codec> {
    match id {
        codec::Id::VP9 => decoder::find_by_name("libvpx-vp9"),
        _ => None,
    }
}
