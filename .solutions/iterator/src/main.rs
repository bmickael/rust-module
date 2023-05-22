fn main() {
    // Ecriture fonctionnelle
    let v: Vec<usize> = (0..=30).filter(|elem| elem % 2 == 0).enumerate().map(|(index, elem)| (elem * 2) + index).collect();
    dbg!(v);
        
    // Ecriture imperative
    const I_BEGIN: usize = 0;
    let mut v = Vec::new();
    let mut i = I_BEGIN;
    while i <= 30 {
        // L'index brut s'obtient par i - I_BEGIN
        if i % 2 == 0 {
            v.push(i * 2 + (i - I_BEGIN) / 2);
        }
        i += 1;
    }
    dbg!(v);
}
