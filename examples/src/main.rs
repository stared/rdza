rdza::rdza! {
    użyj std::kolekcje::Słownik jako Słow;

    cecha KluczWartosc {
        fn zapisz(&sam, klucz: Tekst, wartosc: Tekst);
        fn czytaj(&sam, klucz: Tekst) -> Wynik<Opcja<&Tekst>, Tekst>;
    }

    statyczny zm SLOWNIK: Opcja<Słow<Tekst, Tekst>> = Nic;

    struktura Konkretna;

    impl KluczWartosc dla Konkretna {

        fn zapisz(&sam, klucz: Tekst, wartosc: Tekst) {
            niech słow = niebezpieczny {
                SLOWNIK.pobierz_lub_wstaw_z(Domyślny::domyslny)
            };
            słow.wstaw(klucz, wartosc);
        }

        fn czytaj(&sam, klucz: Tekst) -> Wynik<Opcja<&Tekst>, Tekst> {
            jeśli niech Coś(słow) = niebezpieczny { SLOWNIK.jako_ref() } {
                Dobry(słow.pobierz(&klucz))
            } inaczej {
                Błąd("Pobierz słownik".do())
            }
        }
    }

    pub(skrzynia) fn możliwe(i: u32) -> Opcja<Wynik<u32, Tekst>> {
        jeśli i % 2 == 1 {
            jeśli i == 42 {
                Coś(Błąd(Tekst::z("Cholera")))
            } inaczej {
                Coś(Dobry(33))
            }
        } inaczej {
            Nic
        }
    }

    asynchroniczny fn przykład() {
    }

    asynchroniczny fn przykład2() {
        przykład().czekaj;
    }

    fn main() {
        niech zm x = 31;

        dopasuj x {
            42 => {
                wypisz!("Kotlet schabowy")
            }
            _ => wypisz!("No i działa")
        }

        dla i in 0..10 {
            niech val = pętla {
                przerwij i;
            };

            podczas nie x < val {
                x += 1;
            }

            x = jeśli niech Coś(wynik) = możliwe(i) {
                wynik.rozpakuj()
            } inaczej {
                12
            };
        }

        użyj std::por::Porównanie;
        let _mod7 = vec![0; 100].iter()
            .weź(50)
            .mapuj(|numer| numer %  7)
            .zbierz::<Vec<i32>>()
            .do_iter()
            .złóż(0, |a, numer| match numer.por(&a) {
                Porównanie::Więcej => a - numer,
                Porównanie::Mniej => a + numer,
                Porównanie::Równy => a,
            });
    }
}
