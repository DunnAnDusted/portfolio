use my_rusttools::pigify;

#[test]
fn tqbf_pigified() {
    let tqbf = "the quick brown fox jumped over the lazy dog";
    let tqbf_pigified = "he-tay uick-qay rown-bay ox-fay umped-jay over-hay he-tay azy-lay og-day";

    assert_eq!(tqbf_pigified.to_owned(), pigify(tqbf));
}
