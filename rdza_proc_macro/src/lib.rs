use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Blad" | "Błąd" => "Err",
        "Dobry" => "Ok",
        "Tekst" => "String",
        "Slownik" | "Słownik" => "HashMap",
        "Domyslny" | "Domyślny" => "Default",
        "Awaria" => "Error",
        "Opcja" => "Option",
        "Cos" | "Coś" => "Some",
        "Nic" => "None",
        "Wynik" => "Result",
        "Wlasny" | "Własny" => "Self",
        "kolekcje" => "collections",
        "wypisz" => "println",
        "przerwij" => "break",
        "asynchroniczny" => "async",
        "czekaj" => "await",
        "petla" | "pętla" => "loop",
        "przenies" | "przenieś" => "move",
        "skrzynia" => "crate",
        "Pudlo" | "Pudło" => "Box",
        "nieosiagalny_kod" => "unreachable_code",
        "jako" => "as",
        "stala" | "stała" => "const",
        "cecha" => "trait",
        "typ" => "type",
        "niebezpieczny" => "unsafe",
        "w" => "in",
        "z" => "from",
        "dynamiczny" => "dyn",
        "rozpakuj" => "unwrap",
        "domyslny" | "domyślny" => "default",
        "jako_ref" => "as_ref",
        "we" => "io",
        "zewn" => "extern",
        "falsz" | "fałsz" => "false",
        "funkcja" | "fn" => "fn",
        "nadrzedny" | "nadrzędny" => "super",
        "wstaw" => "insert",

        // iterator functions
        "iter" => "iter",
        "do_iter" => "into_iter",
        "mapuj" => "map",
        "rozszerz" => "flat_map",
        "zloz" | "złóż" => "fold",
        "oproznic" | "opróżnić" => "drain",
        "zbierz" => "collect",
        "znajdz" | "znajdź" => "find",
        "wez" | "weź" => "take", 
        "iloczyn" => "product",

        // ordering
        "por" => "cmp",
        "Porownanie" | "Porównanie" => "Ordering",
        "Wiecej" | "Więcej" => "Greater",
        "Mniej" => "Less",
        "Rowny" | "Równy" => "Equal",
        "pobierz" => "get",
        "zezwol" | "zezwól" => "allow",
        "panika" | "kurde" | "cholera" | "ups" => "panic",
        "modul" | "moduł" => "mod",
        "zm" => "mut",
        "nowy" => "new",
        "gdzie" => "where",
        "dla" => "for",
        "pobierz_lub_wstaw_z" => "get_or_insert_with",
        "main" => "main",
        "pub" => "pub",
        "nie" => None?,
        "zwroc" | "zwróć" => "return",
        "impl" => "impl",
        "ref" => "ref",
        "dopasuj" => "match",
        "jesli" | "jeśli" => "if",
        "inaczej" => "else",
        "sam" => "self",
        "niech" => "let",
        "statyczny" => "static",
        "struktura" => "struct",
        "oczekuj" => "expect",
        "podczas" => "while",
        "uzyj" | "użyj" => "use",
        "do" => "into",
        "prawda" => "true",
        "wyliczenie" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rdza(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
