use std::io::File;

trait MIMEChecker {
  fn classify(&self, data:&Vec<u8>)->Option<(String,String)>;
}

struct ByteMatcher {
  pattern: Vec<u8>,
  mask: Vec<u8>,
  leading_ignore: Vec<u8>,
  content_type: (String,String)
}

impl ByteMatcher {
  fn matches(&self,data:&Vec<u8>)->bool {

    if data.len() < self.pattern.len() {
      return false;
    }
    //TODO replace with iterators if I ever figure them out...
    let mut i = 0u;
    let max_i = data.len()-self.pattern.len();

    loop {

      if !self.leading_ignore.iter().any(|x| *x == data[i]) { break;}

      i=i+1;
      if i>max_i {return false;}
    }

    for j in range(0u,self.pattern.len()) {
      let k = j;
      print!("{}",k);
      if (data[i] & self.mask[j])!=
        (self.pattern[j] & self.mask[j]) {
        return false;
      }
      i=i+1;
    }
    return true;
  }
//TODO These should probably be configured not hard coded
  fn image_x_icon()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x00u8,0x00u8,0x01u8,0x00u8],
      mask:vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("image".to_string(),"x-icon".to_string()),
      leading_ignore:vec![]}
  }
  fn image_x_icon_cursor()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x00u8,0x00u8,0x02u8,0x00u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("image".to_string(),"x-icon".to_string()),
      leading_ignore:vec![]
    }
  }
  fn image_bmp()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x42u8,0x4Du8],
      mask:   vec![0xFFu8,0xFFu8],
      content_type:("image".to_string(),"bmp".to_string()),
      leading_ignore:vec![]
    }
  }
  fn image_gif89a()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x47u8,0x49u8,0x46u8,0x38u8,0x39u8,0x61u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("image".to_string(),"gif".to_string()),
      leading_ignore:vec![]
    }
  }
  fn image_gif87a()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x47u8,0x49u8,0x46u8,0x38u8,0x37u8,0x61u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("image".to_string(),"gif".to_string()),
      leading_ignore:vec![]
    }
  }
  fn image_webp()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x52u8,0x49u8,0x46u8,0x46u8,0x00u8,0x00u8,0x00u8,0x00u8,
                   0x57u8,0x45u8,0x42u8,0x50u8,0x56u8,0x50u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8,0x00u8,0x00u8,0x00u8,0x00u8,
                   0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("image".to_string(),"webp".to_string()),
      leading_ignore:vec![]
    }
  }

  fn image_png()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x89u8,0x50u8,0x4Eu8,0x47u8,0x0Du8,0x0Au8,0x1Au8,0x0Au8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("image".to_string(),"png".to_string()),
      leading_ignore:vec![]
    }
  }
  fn image_jpeg()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0xFFu8,0xD8u8,0xFFu8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8],
      content_type:("image".to_string(),"jpeg".to_string()),
      leading_ignore:vec![]
    }
  }
  fn video_webm()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x1Au8,0x45u8,0xDFu8,0xA3u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("video".to_string(),"webm".to_string()),
      leading_ignore:vec![]
    }
  }
  fn audio_basic()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x2Eu8,0x73u8,0x6Eu8,0x64u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("audio".to_string(),"basic".to_string()),
      leading_ignore:vec![]
    }
  }
  fn audio_aiff()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x46u8,0x4Fu8,0x52u8,0x4Du8,0x00u8,0x00u8,0x00u8,0x00u8,0x41u8,0x49u8,0x46u8,0x46u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8,0x00u8,0x00u8,0x00u8,0x00u8,0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("audio".to_string(),"aiff".to_string()),
      leading_ignore:vec![]
    }
  }
  fn audio_mpeg()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x49u8,0x44u8,0x33u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8],
      content_type:("audio".to_string(),"mpeg".to_string()),
      leading_ignore:vec![]
    }
  }
  fn application_ogg()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x4Fu8,0x67u8,0x67u8,0x53u8,0x00u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("application".to_string(),"ogg".to_string()),
      leading_ignore:vec![]
    }
  }
  fn audio_midi()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x4Du8,0x54u8,0x68u8,0x64u8,0x00u8,0x00u8,0x00u8,0x06u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("audio".to_string(),"midi".to_string()),
      leading_ignore:vec![]
    }
  }
  fn video_avi()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x52u8,0x49u8,0x46u8,0x46u8,0x00u8,0x00u8,0x00u8,0x00u8,
                   0x41u8,0x56u8,0x49u8,0x20u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8,0x00u8,0x00u8,0x00u8,0x00u8,
                   0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("video".to_string(),"avi".to_string()),
      leading_ignore:vec![]
    }
  }
  fn audio_wave()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x52u8,0x49u8,0x46u8,0x46u8,0x00u8,0x00u8,0x00u8,0x00u8,
                   0x57u8,0x41u8,0x56u8,0x45u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8,0x00u8,0x00u8,0x00u8,0x00u8,
                   0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("audio".to_string(),"wave".to_string()),
      leading_ignore:vec![]
    }
  }
  // doctype terminated with Tag terminating (TT) Byte: 0x20 (SP)
  fn text_html_doctype_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x21u8,0x44u8,0x4Fu8,0x43u8,0x54u8,0x59u8,0x50u8,
                   0x45u8,0x20u8,0x48u8,0x54u8,0x4Du8,0x4Cu8,0x20u8],
      mask:   vec![0xFFu8,0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,
                   0xDFu8,0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // doctype terminated with Tag terminating (TT) Byte: 0x3E (">")
  fn text_html_doctype_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x21u8,0x44u8,0x4Fu8,0x43u8,0x54u8,0x59u8,0x50u8,
                   0x45u8,0x20u8,0x48u8,0x54u8,0x4Du8,0x4Cu8,0x3Eu8],
      mask:   vec![0xFFu8,0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,
                   0xDFu8,0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // HTML terminated with Tag terminating (TT) Byte: 0x20 (SP)
  fn text_html_page_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x48u8,0x54u8,0x4Du8,0x4Cu8,0x20u8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // HTML terminated with Tag terminating (TT) Byte: 0x3E (">")
  fn text_html_page_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x48u8,0x54u8,0x4Du8,0x4Cu8,0x3Eu8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // head terminated with Tag Terminating (TT) Byte: 0x20 (SP)
  fn text_html_head_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x48u8,0x45u8,0x41u8,0x44u8,0x20u8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // head terminated with Tag Terminating (TT) Byte: 0x3E (">")
  fn text_html_head_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x48u8,0x45u8,0x41u8,0x44u8,0x3Eu8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // script terminated with Tag Terminating (TT) Byte: 0x20 (SP)
  fn text_html_script_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x53u8,0x43u8,0x52u8,0x49u8,0x50u8,0x54u8,0x20u8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // script terminated with Tag Terminating (TT) Byte: 0x3E (">")
  fn text_html_script_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x53u8,0x43u8,0x52u8,0x49u8,0x50u8,0x54u8,0x3Eu8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // iframe terminated with Tag Terminating (TT) Byte: 0x20 (SP)
  fn text_html_iframe_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x49u8,0x46u8,0x52u8,0x41u8,0x4Du8,0x45u8,0x20u8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // iframe terminated with Tag Terminating (TT) Byte: 0x3E (">")
  fn text_html_iframe_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x49u8,0x46u8,0x52u8,0x41u8,0x4Du8,0x45u8,0x3Eu8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // h1 terminated with Tag Terminating (TT) Byte: 0x20 (SP)
  fn text_html_h1_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x48u8,0x31u8,0x20u8],
      mask:   vec![0xFFu8,0xDFu8,0xFFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // h1 terminated with Tag Terminating (TT) Byte: 0x3E (">")
  fn text_html_h1_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x48u8,0x31u8,0x3Eu8],
      mask:   vec![0xFFu8,0xDFu8,0xFFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // div terminated with Tag Terminating (TT) Byte: 0x20 (SP)
  fn text_html_div_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x44u8,0x49u8,0x56u8,0x20u8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // div terminated with Tag Terminating (TT) Byte: 0x3E (">")
  fn text_html_div_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x44u8,0x49u8,0x56u8,0x3Eu8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // font terminated with Tag Terminating (TT) Byte: 0x20 (SP)
  fn text_html_font_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x46u8,0x4Fu8,0x4Eu8,0x54u8,0x20u8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // font terminated with Tag Terminating (TT) Byte: 0x3E (">")
  fn text_html_font_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x46u8,0x4Fu8,0x4Eu8,0x54u8,0x3Eu8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // table terminated with Tag Terminating (TT) Byte: 0x20 (SP)
  fn text_html_table_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x54u8,0x41u8,0x42u8,0x4Cu8,0x45u8,0x20u8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // table terminated with Tag Terminating (TT) Byte: 0x3E (">")
  fn text_html_table_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x54u8,0x41u8,0x42u8,0x4Cu8,0x45u8,0x3Eu8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // a terminated with Tag Terminating (TT) Byte: 0x20 (SP)
  fn text_html_a_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x41u8,0x20u8],
      mask:   vec![0xFFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // a terminated with Tag Terminating (TT) Byte: 0x3E (">")
  fn text_html_a_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x41u8,0x3Eu8],
      mask:   vec![0xFFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // style terminated with Tag Terminating (TT) Byte: 0x20 (SP)
  fn text_html_style_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x53u8,0x54u8,0x59u8,0x4Cu8,0x45u8,0x20u8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // style terminated with Tag Terminating (TT) Byte: 0x3E (">")
  fn text_html_style_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x53u8,0x54u8,0x59u8,0x4Cu8,0x45u8,0x3Eu8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // title terminated with Tag Terminating (TT) Byte: 0x20 (SP)
  fn text_html_title_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x54u8,0x49u8,0x54u8,0x4Cu8,0x45u8,0x20u8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // title terminated with Tag Terminating (TT) Byte: 0x3E (">")
  fn text_html_title_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x54u8,0x49u8,0x54u8,0x4Cu8,0x45u8,0x3Eu8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // b terminated with Tag Terminating (TT) Byte: 0x20 (SP)
  fn text_html_b_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x42u8,0x20u8],
      mask:   vec![0xFFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // b terminated with Tag Terminating (TT) Byte: 0x3E (">")
  fn text_html_b_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x42u8,0x3Eu8],
      mask:   vec![0xFFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // body terminated with Tag Terminating (TT) Byte: 0x20 (SP)
  fn text_html_body_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x42u8,0x4Fu8,0x44u8,0x59u8,0x20u8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // body terminated with Tag Terminating (TT) Byte: 0x3E (">")
  fn text_html_body_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x42u8,0x4Fu8,0x44u8,0x59u8,0x3Eu8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // br terminated with Tag Terminating (TT) Byte: 0x20 (SP)
  fn text_html_br_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x42u8,0x52u8,0x20u8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // br terminated with Tag Terminating (TT) Byte: 0x3E (">")
  fn text_html_br_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x42u8,0x52u8,0x3Eu8],
      mask:   vec![0xFFu8,0xDFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // p terminated with Tag Terminating (TT) Byte: 0x20 (SP)
  fn text_html_p_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x50u8,0x20u8],
      mask:   vec![0xFFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // p terminated with Tag Terminating (TT) Byte: 0x3E (">")
  fn text_html_p_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x50u8,0x3Eu8],
      mask:   vec![0xFFu8,0xDFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // comment terminated with Tag Terminating (TT) Byte: 0x20 (SP)
  fn text_html_comment_20()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x21u8,0x2Du8,0x2Du8,0x20u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // comment terminated with Tag Terminating (TT) Byte: 0x3E (">")
  fn text_html_comment_3e()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x21u8,0x2Du8,0x2Du8,0x3Eu8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("text".to_string(),"html".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
    }
  }
  // XML
  fn text_xml()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x3Cu8,0x3Fu8,0x78u8,0x6Du8,0x6Cu8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("text".to_string(),"xml".to_string()),
      leading_ignore:vec![0x09u8,0x0Au8,0x0Cu8,0x0Du8,0x20u8]
   }
  }
  // PDF
  fn application_pdf()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x25u8,0x50u8,0x44u8,0x46u8,0x2Du8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("application".to_string(),"pdf".to_string()),
      leading_ignore:vec![]
    }
  }

  fn application_vnd_ms_font_object()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,
                   0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,
                   0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,
                   0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,
                   0x00u8,0x00u8,0x4Cu8,0x50u8],
      mask:   vec![0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,
                   0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,
                   0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,
                   0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,0x00u8,
                   0x00u8,0x00u8,0xFFu8,0xFFu8],
      content_type:("application".to_string(),"vnd.ms-fontobject".to_string()),
      leading_ignore:vec![]
    }
  }

  fn true_type()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x00u8,0x01u8,0x00u8,0x00u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("(TrueType)".to_string(),"".to_string()),
      leading_ignore:vec![]
    }
  }
  fn open_type()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x4Fu8,0x54u8,0x54u8,0x4Fu8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("(OpenType)".to_string(),"".to_string()),
      leading_ignore:vec![]
    }
  }

  fn true_type_collection()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x74u8,0x74u8,0x63u8,0x66u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("(TrueType Collection)".to_string(),"".to_string()),
      leading_ignore:vec![]
    }
  }
  fn application_font_woff()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x77u8,0x4Fu8,0x46u8,0x46u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("application".to_string(),"font-woff".to_string()),
      leading_ignore:vec![]
    }
  }
  fn application_x_gzip()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x1Fu8,0x8Bu8,0x08u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8],
      content_type:("application".to_string(),"x-gzip".to_string()),
      leading_ignore:vec![]
    }
  }
  fn application_zip()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x50u8,0x4Bu8,0x03u8,0x04u8],
     mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("application".to_string(),"zip".to_string()),
      leading_ignore:vec![]
    }
  }
  fn application_x_rar_compressed()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x52u8,0x61u8,0x72u8,0x20u8,0x1Au8,0x07u8,0x00u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8],
      content_type:("application".to_string(),"x-rar-compressed".to_string()),
      leading_ignore:vec![]
    }
  }
  fn application_postscript()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0x25u8,0x21u8,0x50u8,0x53u8,0x2Du8,0x41u8,0x64u8,0x6Fu8,
                   0x62u8,0x65u8,0x2Du8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8,0xFFu8,
                   0xFFu8,0xFFu8,0xFFu8],
      content_type:("application".to_string(),"postscript".to_string()),
      leading_ignore:vec![]
    }
  }
  fn text_plain_utf_16be_bom()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0xFEu8,0xFFu8,0x00u8,0x00u8],
      mask:   vec![0xFFu8,0xFFu8,0x00u8,0x00u8],
      content_type:("text".to_string(),"plain".to_string()),
      leading_ignore:vec![]
    }
  }
  fn text_plain_utf_16le_bom()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0xFFu8,0xFEu8,0x00u8,0x00u8],
      mask:   vec![0xFFu8,0xFFu8,0x00u8,0x00u8],
      content_type:("text".to_string(),"plain".to_string()),
      leading_ignore:vec![]
    }
  }

  fn text_plain_utf_8_bom()->ByteMatcher {
    return ByteMatcher{
      pattern:vec![0xEFu8,0xBBu8,0xBFu8,0x00u8],
      mask:   vec![0xFFu8,0xFFu8,0xFFu8,0x00u8],
      content_type:("text".to_string(),"plain".to_string()),
      leading_ignore:vec![]
    }
  }
}

impl MIMEChecker for ByteMatcher {
  fn classify(&self, data:&Vec<u8>)->Option<(String,String)>
  {
   return if self.matches(data) {
      Some(self.content_type.clone())
    } else {
      None
    };
  }
}

struct Mp4Matcher;

impl Mp4Matcher {
  fn matches(&self,data:&Vec<u8>)->bool {
    if data.len() < 12 {return false;}
    let box_size = ((data[0] as u32)<<3 | (data[1] as u32)<<2 |(data[2] as u32)<<1|(data[3] as u32)) as uint;
    if (data.len()<box_size) || (box_size%4!=0) {return false;}
    //TODO replace with iterators
    let ftyp = [0x66,0x74,0x79,0x70];
    let mp4 =  [0x6D,0x70,0x34];

    for i in range(4u,8u) {
      if data[i]!=ftyp[i-4] {
        return false;
      }
    }
    let mut all_match = true;
    for i in range(8u,11u) {
      if data[i]!=mp4[i-8u] {all_match = false; break;}
    }
    if all_match {return true;}
    let mut bytes_read = 16u;

    while bytes_read < box_size
    {
      all_match = true;
      for i in range(0u,3u) {
        if mp4[i]!=data[i+bytes_read] {all_match=false; break;}
      }
      if all_match {return true;}
      bytes_read=bytes_read+4;
    }
    return false;
  }

}
impl MIMEChecker for Mp4Matcher {
  fn classify(&self, data:&Vec<u8>)->Option<(String,String)> {
   return if self.matches(data) {
      Some(("video".to_string(), "mp4".to_string()))
    } else {
      None
    };
  }
}

struct MIMEClassifier {
  //TODO Replace with boxed trait
  byte_matchers: Vec<Box<MIMEChecker+Send>>,
}

impl MIMEClassifier {
  fn new()->MIMEClassifier {
     //TODO These should be configured from a settings file
     //     and not hardcoded

     let mut ret = MIMEClassifier{byte_matchers:Vec::new()};
     ret.byte_matchers.push(box ByteMatcher::image_x_icon());
     ret.byte_matchers.push(box ByteMatcher::image_x_icon_cursor());
     ret.byte_matchers.push(box ByteMatcher::image_bmp());
     ret.byte_matchers.push(box ByteMatcher::image_gif89a());
     ret.byte_matchers.push(box ByteMatcher::image_gif87a());
     ret.byte_matchers.push(box ByteMatcher::image_webp());
     ret.byte_matchers.push(box ByteMatcher::image_png());
     ret.byte_matchers.push(box ByteMatcher::image_jpeg());
     ret.byte_matchers.push(box ByteMatcher::video_webm());
     ret.byte_matchers.push(box ByteMatcher::audio_basic());
     ret.byte_matchers.push(box ByteMatcher::audio_aiff());
     ret.byte_matchers.push(box ByteMatcher::audio_mpeg());
     ret.byte_matchers.push(box ByteMatcher::application_ogg());
     ret.byte_matchers.push(box ByteMatcher::audio_midi());
     ret.byte_matchers.push(box ByteMatcher::video_avi());
     ret.byte_matchers.push(box ByteMatcher::audio_wave());
     ret.byte_matchers.push(box ByteMatcher::application_font_woff());
     ret.byte_matchers.push(box ByteMatcher::true_type_collection());
     ret.byte_matchers.push(box ByteMatcher::open_type());
     ret.byte_matchers.push(box ByteMatcher::true_type());
     ret.byte_matchers.push(box ByteMatcher::application_vnd_ms_font_object());
     ret.byte_matchers.push(box ByteMatcher::application_x_gzip());
     ret.byte_matchers.push(box ByteMatcher::application_zip());
     ret.byte_matchers.push(box ByteMatcher::application_x_rar_compressed());
     ret.byte_matchers.push(box ByteMatcher::text_plain_utf_8_bom());
     ret.byte_matchers.push(box ByteMatcher::text_plain_utf_16le_bom());
     ret.byte_matchers.push(box ByteMatcher::text_plain_utf_16be_bom());
     ret.byte_matchers.push(box ByteMatcher::application_postscript());
     ret.byte_matchers.push(box ByteMatcher::text_html_doctype_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_doctype_3e());
     ret.byte_matchers.push(box ByteMatcher::text_html_page_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_page_3e());
     ret.byte_matchers.push(box ByteMatcher::text_html_head_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_head_3e());
     ret.byte_matchers.push(box ByteMatcher::text_html_script_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_script_3e());
     ret.byte_matchers.push(box ByteMatcher::text_html_iframe_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_iframe_3e());
     ret.byte_matchers.push(box ByteMatcher::text_html_h1_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_h1_3e());
     ret.byte_matchers.push(box ByteMatcher::text_html_div_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_div_3e());
     ret.byte_matchers.push(box ByteMatcher::text_html_font_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_font_3e());
     ret.byte_matchers.push(box ByteMatcher::text_html_table_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_table_3e());
     ret.byte_matchers.push(box ByteMatcher::text_html_a_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_a_3e());
     ret.byte_matchers.push(box ByteMatcher::text_html_style_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_style_3e());
     ret.byte_matchers.push(box ByteMatcher::text_html_title_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_title_3e());
     ret.byte_matchers.push(box ByteMatcher::text_html_b_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_b_3e());
     ret.byte_matchers.push(box ByteMatcher::text_html_body_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_body_3e());
     ret.byte_matchers.push(box ByteMatcher::text_html_br_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_br_3e());
     ret.byte_matchers.push(box ByteMatcher::text_html_p_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_p_3e());
     ret.byte_matchers.push(box ByteMatcher::text_html_comment_20());
     ret.byte_matchers.push(box ByteMatcher::text_html_comment_3e());
     ret.byte_matchers.push(box ByteMatcher::text_xml());
     ret.byte_matchers.push(box ByteMatcher::application_pdf());
     ret.byte_matchers.push(box Mp4Matcher);
     return ret;

  }

  fn classify(&self,data:&Vec<u8>)->Option<(String,String)> {
    for matcher in self.byte_matchers.iter()
    {
      match matcher.classify(data)
      {
        Some(mime)=>{ return Some(mime);}
        None=>{}
      }
    }
    return None;
  }
}

#[test]
fn test_sniff_mp4() {
  let matcher = Mp4Matcher;

  let p = Path::new("./tests/content/parsable_mime/video/mp4/test.mp4");
  let mut file = File::open(&p);
  let read_result = file.read_to_end();
  match read_result {
    Ok(data) => {
      println!("Data Length {:u}",data.len());
      if !matcher.matches(&data) {
        panic!("Didn't read mime type")
      }
    },
    Err(e) => panic!("Couldn't read from file with error {}",e)
  }
}

#[cfg(test)]
fn test_classification_full(filename_orig:&Path,type_string:&str,subtype_string:&str){

  let mut filename = Path::new("./tests/content/parsable_mime/");

  filename.push(filename_orig);

  let classifier = MIMEClassifier::new();

  let mut file = File::open(&filename);
  let read_result = file.read_to_end();
  match read_result {
    Ok(data) => {
      match classifier.classify(&data)
      {
        Some(mime)=>{
          let parsed_type=mime.ref0().clone();
          let parsed_subtp=mime.ref1().clone();
           if (parsed_type!=type_string.to_string())||
              (parsed_subtp!=subtype_string.to_string()) {
            panic!("File {} parsed incorrectly should be {}/{}, parsed as {}/{}",
              filename.as_str(),type_string,subtype_string,parsed_type,
              parsed_subtp);
          }
        }
        None=>{panic!("No classification found for {}",filename.as_str());}
      }
    }
    Err(e) => {panic!("Couldn't read from file {} with error {}",filename.as_str(),e);}
  }
}

#[cfg(test)]
fn test_classification(file:&str,type_string:&str,subtype_string:&str){
  let mut x = Path::new("./");
  x.push(type_string);
  x.push(subtype_string);
  x.push(file);
  test_classification_full(&x,type_string,subtype_string);
}

#[test]
fn test_classification_x_icon() { test_classification("test.ico","image","x-icon"); }

#[test]
fn test_classification_x_icon_cursor() {
 test_classification("test_cursor.ico","image","x-icon");
}

#[test]
fn test_classification_bmp() { test_classification("test.bmp","image","bmp"); }

#[test]
fn test_classification_gif87a() {
  test_classification("test87a.gif","image","gif");
}

#[test]
fn test_classification_gif89a() {
  test_classification("test89a.gif","image","gif");
}

#[test]
fn test_classification_webp() {
  test_classification("test.webp","image","webp");
}

#[test]
fn test_classification_png() {
  test_classification("test.png","image","png");
}

#[test]
fn test_classification_jpg() {
  test_classification("test.jpg","image","jpeg");
}

#[test]
fn test_classification_webm() {
  test_classification("test.webm","video","webm");
}

#[test]
fn test_classification_mp4() {
  test_classification("test.mp4","video","mp4");
}

#[test]
fn test_classification_avi() {
  test_classification("test.avi","video","avi");
}

#[test]
fn test_classification_basic() {
  test_classification("test.au","audio","basic");
}

#[test]
fn test_classification_aiff() {
  test_classification("test.aif","audio","aiff");
}

#[test]
fn test_classification_mpeg() {
  test_classification("test.mp3","audio","mpeg");
}

#[test]
fn test_classification_midi() {
  test_classification("test.mid","audio","midi");
}

#[test]
fn test_classification_wave() {
  test_classification("test.wav","audio","wave");
}

#[test]
fn test_classification_ogg() {
  test_classification("small.ogg","application","ogg");
}

#[test]
fn test_classification_vsn_ms_fontobject() {
  test_classification("vnd.ms-fontobject","application","vnd.ms-fontobject");
}

#[test]
fn test_true_type() {
  test_classification_full(&Path::new("unknown/true_type.ttf"),"(TrueType)","");
}

#[test]
fn test_open_type() {
  test_classification_full(&Path::new("unknown/open_type"),"(OpenType)","");
}

#[test]
fn test_classification_true_type_collection() {
  test_classification_full(&Path::new("unknown/true_type_collection.ttc"),"(TrueType Collection)","");
}

#[test]
fn test_classification_woff() {
  test_classification("test.wof","application","font-woff");
}

#[test]
fn test_classification_gzip() {
  test_classification("test.gz","application","x-gzip");
}

#[test]
fn test_classification_zip() {
  test_classification("test.zip","application","zip");
}

#[test]
fn test_classification_rar() {
  test_classification("test.rar","application","x-rar-compressed");
}

#[test]
fn test_text_html_doctype_20() {
  test_classification("text_html_doctype_20.html","text","html");
  test_classification("text_html_doctype_20_u.html","text","html");
}
#[test]
fn test_text_html_doctype_3e() {
  test_classification("text_html_doctype_3e.html","text","html");
  test_classification("text_html_doctype_3e_u.html","text","html");
}

#[test]
fn test_text_html_page_20() {
  test_classification("text_html_page_20.html","text","html");
  test_classification("text_html_page_20_u.html","text","html");
}

#[test]
fn test_text_html_page_3e() {
  test_classification("text_html_page_3e.html","text","html");
  test_classification("text_html_page_3e_u.html","text","html");
}
#[test]
fn test_text_html_head_20() {
  test_classification("text_html_head_20.html","text","html");
  test_classification("text_html_head_20_u.html","text","html");
}

#[test]
fn test_text_html_head_3e() {
  test_classification("text_html_head_3e.html","text","html");
  test_classification("text_html_head_3e_u.html","text","html");
}
#[test]
fn test_text_html_script_20() {
  test_classification("text_html_script_20.html","text","html");
  test_classification("text_html_script_20_u.html","text","html");
}

#[test]
fn test_text_html_script_3e() {
  test_classification("text_html_script_3e.html","text","html");
  test_classification("text_html_script_3e_u.html","text","html");
}
#[test]
fn test_text_html_iframe_20() {
  test_classification("text_html_iframe_20.html","text","html");
  test_classification("text_html_iframe_20_u.html","text","html");
}

#[test]
fn test_text_html_iframe_3e() {
  test_classification("text_html_iframe_3e.html","text","html");
  test_classification("text_html_iframe_3e_u.html","text","html");
}
#[test]
fn test_text_html_h1_20() {
  test_classification("text_html_h1_20.html","text","html");
  test_classification("text_html_h1_20_u.html","text","html");
}

#[test]
fn test_text_html_h1_3e() {
  test_classification("text_html_h1_3e.html","text","html");
  test_classification("text_html_h1_3e_u.html","text","html");
}
#[test]
fn test_text_html_div_20() {
  test_classification("text_html_div_20.html","text","html");
  test_classification("text_html_div_20_u.html","text","html");
}

#[test]
fn test_text_html_div_3e() {
  test_classification("text_html_div_3e.html","text","html");
  test_classification("text_html_div_3e_u.html","text","html");
}
#[test]
fn test_text_html_font_20() {
  test_classification("text_html_font_20.html","text","html");
  test_classification("text_html_font_20_u.html","text","html");
}

#[test]
fn test_text_html_font_3e() {
  test_classification("text_html_font_3e.html","text","html");
  test_classification("text_html_font_3e_u.html","text","html");
}
#[test]
fn test_text_html_table_20() {
  test_classification("text_html_table_20.html","text","html");
  test_classification("text_html_table_20_u.html","text","html");
}

#[test]
fn test_text_html_table_3e() {
  test_classification("text_html_table_3e.html","text","html");
  test_classification("text_html_table_3e_u.html","text","html");
}
#[test]
fn test_text_html_a_20() {
  test_classification("text_html_a_20.html","text","html");
  test_classification("text_html_a_20_u.html","text","html");
}

#[test]
fn test_text_html_a_3e() {
  test_classification("text_html_a_3e.html","text","html");
  test_classification("text_html_a_3e_u.html","text","html");
}
#[test]
fn test_text_html_style_20() {
  test_classification("text_html_style_20.html","text","html");
  test_classification("text_html_style_20_u.html","text","html");
}

#[test]
fn test_text_html_style_3e() {
  test_classification("text_html_style_3e.html","text","html");
  test_classification("text_html_style_3e_u.html","text","html");
}
#[test]
fn test_text_html_title_20() {
  test_classification("text_html_title_20.html","text","html");
  test_classification("text_html_title_20_u.html","text","html");
}

#[test]
fn test_text_html_title_3e() {
  test_classification("text_html_title_3e.html","text","html");
  test_classification("text_html_title_3e_u.html","text","html");
}
#[test]
fn test_text_html_b_20() {
  test_classification("text_html_b_20.html","text","html");
  test_classification("text_html_b_20_u.html","text","html");
}

#[test]
fn test_text_html_b_3e() {
  test_classification("text_html_b_3e.html","text","html");
  test_classification("text_html_b_3e_u.html","text","html");
}
#[test]
fn test_text_html_body_20() {
  test_classification("text_html_body_20.html","text","html");
  test_classification("text_html_body_20_u.html","text","html");
}

#[test]
fn test_text_html_body_3e() {
  test_classification("text_html_body_3e.html","text","html");
  test_classification("text_html_body_3e_u.html","text","html");
}
#[test]
fn test_text_html_br_20() {
  test_classification("text_html_br_20.html","text","html");
  test_classification("text_html_br_20_u.html","text","html");
}

#[test]
fn test_text_html_br_3e() {
  test_classification("text_html_br_3e.html","text","html");
  test_classification("text_html_br_3e_u.html","text","html");
}
#[test]
fn test_text_html_p_20() {
  test_classification("text_html_p_20.html","text","html");
  test_classification("text_html_p_20_u.html","text","html");
}
#[test]
fn test_text_html_p_3e() {
  test_classification("text_html_p_3e.html","text","html");
  test_classification("text_html_p_3e_u.html","text","html");
}

#[test]
fn test_text_html_comment_20() {
  test_classification("text_html_comment_20.html","text","html");
}

#[test]
fn test_text_html_comment_3e() {
  test_classification("text_html_comment_3e.html","text","html");
}

#[test]
fn test_xml() {
  test_classification("test.xml","text","xml");
}

#[test]
fn test_pdf() {
  test_classification("test.pdf","application","pdf");
}

#[test]
fn test_postscript() {
  test_classification("test.ps","application","postscript");
}

#[test]
fn test_utf_16be_bom() {
  test_classification("utf16bebom.txt","text","plain");
}

#[test]
fn test_utf_16le_bom() {
  test_classification("utf16lebom.txt","text","plain");
}

#[test]
fn test_utf_8_bom() {
  test_classification("utf8bom.txt","text","plain");
}

#[test]
fn test_rss_feed() {
  test_classification_full(&Path::new("text/xml/feed.rss"),"application","rss+xml")
}

#[test]
fn test_atom_feed() {
  test_classification_full(&Path::new("text/xml/feed.atom"),"application","atom+xml")
}