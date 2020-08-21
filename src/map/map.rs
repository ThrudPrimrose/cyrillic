use std::collections::HashMap;

pub struct FinalStates<'a> {
    map : HashMap<&'a str,&'a str>
}

impl FinalStates<'static> {
    pub fn copy_get(&self, i : &str) -> Option<&str> {
        //self.map.get(i)?.clone();
        let opt = self.map.get(i);
        match opt {
            Some(s) => return Some(s.clone()),
            None => return None
        }
        
    }

    pub fn convertible(&self, s : &str) -> bool {
        for ch in s.chars() {
            let image = ch.to_string();
            let val = self.map.get::<str>(&image);
            match val {
                None => continue,
                Some(_) => return true
            }
        }
        return false
    }

    pub fn convert(&self, s : &str) -> String {
        let mut tmp = String::from("");
        for ch in s.chars() {
            let image = ch.to_string();
            let val = self.map.get::<str>(&image);
            match val {
                None => tmp.push(ch),
                Some(s) => tmp.push_str(s)
            }
        }
        return tmp 
    }

    pub fn new() -> Self {
        FinalStates {
            map : get_map()
        }
    }
}

fn get_map() -> HashMap<&'static str,&'static str> {
    let tret: HashMap<&'static str, &'static str> = 
    [
        ("А","A"), 
        ("а","a"), 
        ("Б","B"), 
        ("б","b"), 
        ("В","V"), 
        ("в","v"), 
        ("Г","G"), 
        ("г","g"), 
        ("Д","D"), 
        ("д","d"),
        ("Е","E"), 
        ("е","e"), 
        ("Ё","Yo"), 
        ("ё","yo"), 
        ("Ж","Zh"), 
        ("ж","zh"), 
        ("З","Z"), 
        ("з","z"), 
        ("И","I"),
        ("и","i"), 
        ("Й","J"),
        ("й","j"), 
        ("К","K"), 
        ("к","k"), 
        ("Л","L"), 
        ("л","l"), 
        ("М","M"), 
        ("м","m"), 
        ("Н","N"), 
        ("н","n"), 
        ("О","O"), 
        ("о","o"),
        ("П","P"),
        ("п","p"), 
        ("Р","R"), 
        ("р","r"),
        ("С","S"),  
        ("с","s"), 
        ("Т","T"), 
        ("т","t"), 
        ("У","U"), 
        ("у","u"), 
        ("Ф","F"), 
        ("ф","f"), 
        ("Х","H"), 
        ("х","h"), 
        ("Ц","Ts"), 
        ("ц","ts"), 
        ("Ч","Ch"), 
        ("ч","ch"), 
        ("Ш","Sh"), 
        ("ш","sh"), 
        ("Щ","Shch"), 
        ("щ","shch"), 
        ("Ъ",""), 
        ("ъ",""), 
        ("Ы",""), 
        ("ы",""), 
        ("Ь",""), 
        ("ь",""), 
        ("Э","E"), 
        ("э","e"), 
        ("Ю","Yu"), 
        ("ю","yu"), 
        ("Я","Ya"), 
        ("я","ya")
    ].iter().cloned().collect();
    return tret;
}

