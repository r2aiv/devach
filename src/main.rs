use rand::Rng;

fn main() 
{
    let alpha_glasn = "АЕЁИОУЭЮЯЫ";
    let aplha_sogl = "БВГДДЗКЛМНПРСТФХЦЧШЩ";

    let mut rng = rand::thread_rng();

    let mut alpha_glasn_vec = Vec::new();
    let mut alpha_sogl_vec = Vec::new();

    let mut dest_string = "ДЕ-ВАЧ-".to_string();

    for chr in alpha_glasn.chars()
    {
        alpha_glasn_vec.push(chr);
    }

    for chr in aplha_sogl.chars()
    {
        alpha_sogl_vec.push(chr);
    }

    let rand_glasn = rng.gen::<usize>() % alpha_glasn_vec.len();
    let rand_sogl = rng.gen::<usize>() % alpha_sogl_vec.len();

    dest_string.push(alpha_sogl_vec[rand_sogl]);
    dest_string.push(alpha_glasn_vec[rand_glasn]);

    println!("{}", dest_string);    
}

