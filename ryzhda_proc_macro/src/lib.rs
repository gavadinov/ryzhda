use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Греш" => "Err",
        "Добре" => "Ok",
        "Низ" => "String",
        "Речник" => "HashMap",
        "ПоПодразбиране" => "Default",
        "Грешка" => "Error",
        "Опция" => "Option",
        "Нещо" => "Some",
        "Нищо" => "None",
        "Резултат" => "Result",
        "СебеСи" => "Self",
        "изписванелин" => "println",
        "прекъсване" => "break",
        "асинх" => "async",
        "изчакване" => "await",
        "цикъл" => "loop",
        "преместване" => "move",
        "щайга" => "crate",
        "недостъпен_код" => "unreachable_code",
        "като" => "as",
        "константа" => "const",
        "описание" => "trait",
        "опасно" => "unsafe",
        "в" => "in",
        "от" => "from",
        "динамично" => "dyn",
        "разопаковане" => "unwrap",
        "по_подразбиране" => "default",
        "като_референция" => "as_ref",
        "ви" => "io",
        "външна" => "extern",
        "грешно" => "false",
        "функция" => "fn",
        "супер" => "super",
        "вмъкване" => "insert",
        "взимане" => "get",
        "разрешаване" => "allow",
        "опа" | "паника" | "деба" => "panic",
        "модул" => "mod",
        "изменяема" => "mut",
        "ново" => "new",
        "където" => "where",
        "за" => "for",
        "взимане_или_вмъкване_с" => "get_or_insert_with",
        "главна" => "main",
        "публично" => "pub",
        "какво" => None?,
        "върни" => "return",
        "реализация" => "impl",
        "референция" => "ref",
        "съвпадение" => "match",
        "ако" => "if",
        "или" => "else",
        "себе_си" => "self",
        "нека" => "let",
        "статичнa" => "static",
        "структура" => "struct",
        "очаквано" => "expect",
        "докато" => "while",
        "използвай" => "use",
        "във" => "into",
        "вярно" => "true",
        "изброяване" => "enum",

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
pub fn ryzhda(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
