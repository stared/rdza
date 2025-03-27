# rdza

<p align="center"><img src="logo.jpg" alt="Rust Polish Logo"></p>

Czy nie jesteś _zmęczony_ pisaniem programów w Ruscie po angielsku? Lubisz często mówić
"cholera"? Chciałbyś spróbować czegoś innego, w egzotycznym i
zabawnie brzmiącym języku? Chciałbyś dodać polski charakter do swoich
programów?

**rdza** (polskie słowo na _Rust_) jest tutaj, aby uratować Twój dzień, ponieważ pozwala Ci
pisać programy w Ruscie po polsku, używając polskich słów kluczowych, polskich nazw funkcji,
polskich idiomów.

Nie czujesz się komfortowo używając tylko polskich słów? Nie martw się!
Polski Rust jest w pełni kompatybilny z angielskim Rustem, więc możesz mieszać oba według
własnego uznania.

Oto przykład tego, co można osiągnąć z Rdzą:

## struct and impl (czyli Struktura i Implementacja)

```rust
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
}
```

## Inne przykłady

Zobacz [przykłady](./examples/src/main.rs), aby zobaczyć jak działa cała
składnia. Bardzo dobrze!

## ale po co to robić?

- [Francuzi](https://github.com/bnjbvr/rouille) i [Holendrzy](https://github.com/jeroenhd/roest) to potrafią, więc my też!

## Współpraca

Przede wszystkim, _dziękuję bardzo_ za rozważenie udziału w tym żarcie,
polski rząd będzie ci wdzięczny później! Nie krępuj się dodawać identyfikatory
tu i tam, i otwórz pull-request do `główneygałęzi` (polskie słowo na
`main branch`). Początkowe tłumaczenie zostało wykonane przez [Shemnei](https://github.com/Shemnei/) i [michidk](https://github.com/michidk/).

## Licencja

[WTFPL](http://www.wtfpl.net/). Obrazy nie są objęte tą licencją, patrz poniżej.

Atrybucje obrazów:

- "Brezel und Filzhut zum Oktoberfest" autorstwa Tim Reckmann | a59.de na licencji CC BY 2.0
- "Lederhose" na licencji CC BY-NC-SA 4.0
