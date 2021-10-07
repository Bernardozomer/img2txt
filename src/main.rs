use image;

// Taken from Ashis Kumar Sahoo @ https://ashiskumar.com/create-ascii-gradient/.
const GRADIENT: &str = " `´¨·…¸ˆ˜’‚‘’:;›‹¹­°–²º³~“”„ª^—¯¦÷¡!|¬•«»/\\)(+{*}×?¿[†7i><ìï=íl1vjrîcotI¤%ƒJz‡¼½‰u¢sòön@CóÌÏ3±LÍV0£Y©wçù2™üa&úô5õ4f¾ÎxyOµ6eTUž9S®ŸkÝ§ûhàÇšäPZáñDF¥bÒÿåÖÞÓèýÙdë$ÜÚ8éðø¶Gâãmpß€AÔÕŠÛqêÐgXœŽRKHÀÄEþÁN#æÅBQWÂÃÈËÉMØŒÊÑÆ"; 

fn main() {
    let mut gradient: Vec<char> = vec![];
    for c in GRADIENT.chars() {
        gradient.push(c);
    }

    let img = image::open("itch-smaller.png").unwrap().to_luma8();

    for (x, _y, luma) in img.enumerate_pixels() {
        let normalized_luminance = luma.0[0] as f64 / 255.0 ;
        let char_index = normalized_luminance * ((gradient.len() - 1) as f64);

        print!("{}", gradient[char_index as usize]);

        if x == img.width() - 1 {
            println!();
        }
    }
}
