use std::io::Write; //println!("{:?} bytes written to {}", std::fs::OpenOptions::new().truncate(true).write(true).open(&path).unwrap().write(&text.as_bytes()).unwrap(), &path);

fn main() {
 let     args    : Vec<String> = std::env::args().collect() ;
 let     file    : &String     = &args[1].trim().to_string();
 let mut first   : f32         = 14.4f32                    ;
 let     folder  : &String     = &args[3].trim().to_string();
 let mut forward : f32         = 30f32                      ;
 let mut index   : f32         = 0f32                       ;
 let mut last    : f32         = 0.72f32                    ;
 let mut reverse : f32         = 6f32                       ;
 let mut splitter: f32         = 1f32                       ;
 let mut text    : String      = String::new()              ;

 match &args[2].parse::<f32>() { Ok(f32) => { first    = *f32 } Err(error) => { println!("error: {:?}", error); } }
 match &args[4].parse::<f32>() { Ok(f32) => { forward  = *f32 } Err(error) => { println!("error: {:?}", error); } }
 match &args[5].parse::<f32>() { Ok(f32) => { last     = *f32 } Err(error) => { println!("error: {:?}", error); } }
 match &args[6].parse::<f32>() { Ok(f32) => { reverse  = *f32 } Err(error) => { println!("error: {:?}", error); } }
 match &args[7].parse::<f32>() { Ok(f32) => { splitter = *f32 } Err(error) => { println!("error: {:?}", error); } }

 let step: f32 = ((first / last).powf(1f32 / (forward - 1f32)) * 10000f32).round() / 10000f32;                                        

 while index < forward {
  let ratio: f32 = (last * step.powf(forward - 1f32 - index) * 10000f32).round() / 10000f32;                                          

  if ratio < 10f32 {
   text = text.clone() + &"\n".to_string() + "ratios_forward[" + &format!("{:02}", index)[..] + "]: 0" + &format!("{:.4}", ratio)[..];

  } else {//if ratio < 10f32 {
   text = text.clone() + &"\n".to_string() + "ratios_forward[" + &format!("{:02}", index)[..] + "]: " + &format!("{:.4}", ratio)[..];

  }//} else {//if ratio < 10f32 {

  index = index + 1f32;
 }//while index < forward {

 text = text.clone() + &"\n".to_string();

 if splitter.abs() < 2f32 {
  index = 0f32;

  while index < reverse {
   let ratio: f32 = (last * step.powf(forward - 1f32 - index) * 10000f32).round() / 10000f32;                                          

   if ratio < 10f32 {
    text = text.clone() + &"\n".to_string() + "ratios_reverse[" + &format!("{:01}", index)[..] + "]: -0" + &format!("{:.4}", ratio)[..];              

   } else {//if ratio < 10f32 {
    text = text.clone() + &"\n".to_string() + "ratios_reverse[" + &format!("{:01}", index)[..] + "]: -" + &format!("{:.4}", ratio)[..];              

   }//} else {//if ratio < 10f32 {

   index = index + 1f32;                                                                                                      
  }//while index < reverse {

 } else {//if splitter.abs() < 2f32 {
  let mut ratio: f32 = (last * step.powf(forward - 1f32) * 10000f32).round() / 10000f32;                                          

  if ratio < 10f32 {
   text = text.clone() + &"\n".to_string() + "ratios_reverse[0]: -0" + &format!("{:.4}", ratio)[..];              

  } else {//if ratio < 10f32 {
   text = text.clone() + &"\n".to_string() + "ratios_reverse[0]: -" + &format!("{:.4}", ratio)[..];              

  }//} else {//if ratio < 10f32 {

  ratio = (last * step.powf(forward - 2f32) * 10000f32).round() / 10000f32;                                          

  if ratio < 10f32 {
   text = text.clone() + &"\n".to_string() + "ratios_reverse[1]: -0" + &format!("{:.4}", ratio)[..];              

  } else {//if ratio < 10f32 {
   text = text.clone() + &"\n".to_string() + "ratios_reverse[1]: -" + &format!("{:.4}", ratio)[..];              

  }//} else {//if ratio < 10f32 {

  ratio = (last * step.powf(forward / 2f32 - 1f32) * 10000f32).round() / 10000f32;                                          

  if ratio < 10f32 {
   text = text.clone() + &"\n".to_string() + "ratios_reverse[2]: -0" + &format!("{:.4}", ratio)[..];              

  } else {//if ratio < 10f32 {
   text = text.clone() + &"\n".to_string() + "ratios_reverse[2]: -" + &format!("{:.4}", ratio)[..];              

  }//} else {//if ratio < 10f32 {

  ratio = (last * step.powf(forward / 2f32 - 2f32) * 10000f32).round() / 10000f32;                                          

  if ratio < 10f32 {
   text = text.clone() + &"\n".to_string() + "ratios_reverse[3]: -0" + &format!("{:.4}", ratio)[..];              

  } else {//if ratio < 10f32 {
   text = text.clone() + &"\n".to_string() + "ratios_reverse[3]: -" + &format!("{:.4}", ratio)[..];              

  }//} else {//if ratio < 10f32 {
 }//} else {//if splitter.abs() < 2f32 {

 text = text.clone() + &"\n".to_string();

 if !std::path::Path::new(folder).exists() {
  match std::fs::create_dir(folder) {
   Ok(result) => { println!("result: {:?}", result); }
   Err(error) => { println!("error: {:?}", error); }
  }//match std::fs::create_dir(folder) {
 }//if !std::path::Path::new(folder).exists() {

 let path: String = folder.to_owned() + if (&folder[..]).chars().nth(folder.trim().len()) == Some('\\') { "" } else { "\\" } + file;

 if !std::path::Path::new(&path).exists() {
  match std::fs::File::create(path.clone()) {
   Ok(result) => { println!("result: {:?}", result); }
   Err(error) => { println!("error: {:?}", error); }
  }//match std::fs::File::create(path.clone()) {
 }//if !std::path::Path::new(&path).exists() {

 println!("{:?} bytes written to {}", std::fs::OpenOptions::new().truncate(true).write(true).open(&path).unwrap().write(&text.as_bytes()).unwrap(), &path);
}//fn main() {
