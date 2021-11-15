# ryzhda

![](https://raw.githubusercontent.com/gavadinov/ryzhda/основен/logo.jpg)

Aren't you _изморен_ from writing Rust programs in English? Do you like saying
"опаа" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Bulgarian touch to your
programs?

**ryzhda** (Bulgarian for _Rust_) is here to save your day, as it allows you to
write Rust programs in Bulgarian, using Bulgarian keywords, Bulgarian function names,
Bulgarian idioms.

Here's an example of what can be achieved with Ryzhda:

### trait and impl (aka описание и реализация)

```rust
ryzhda::ryzhda! {
    външна щайга ryzhda;

    използвай std::collections::Речник;

    описание КлючСтойност {
        функция писане(&себе_си, ключ: Низ, стойност: Низ);
        функция взимане(&себе_си, ключ: Низ) -> Резултат<Опция<&Низ>, Низ>;
    }

    статичнa изменяема РЕЧНИК: Опция<Речник<Низ, Низ>> = Нищо;
    структура Конкретна;
    реализация КлючСтойност за Конкретна {
        функция писане(&себе_си, ключ: Низ, стойност: Низ) {
            нека речн = опасно { РЕЧНИК.взимане_или_вмъкване_с(ПоПодразбиране::по_подразбиране) };
            речн.вмъкване(ключ, стойност);
        }
        функция взимане(&себе_си, ключ: Низ) -> Резултат<Опция<&Низ>, Низ> {
            ако нека Нещо(речн) = опасно { РЕЧНИК.като_референция() } {
                Добре(речн.взимане(&ключ))
            } или {
                Греш("липсва в речник".във())
            }
        }
    }

    публично(щайга) функция може_би(i: u32) -> Опция<Резултат<u32, Низ>> {
        ако i % 2 == 1 {
            ако i == 42 {
                Нещо(Греш(Низ::от("опа")))
            } или {
                Нещо(Добре(33))
            }
        } или {
            Нищо
        }
    }

    асинх функция пример() {}

    асинх функция пример2() {
        пример().изчакване;
    }

    функция главна() {
        нека изменяема x = 31;
        съвпадение x {
            42 => {
                изписванелин!("хубава работа");
            }
            _ => {
                изписванелин!("а такааа");
            }
        }

        за i в 0..10 {
            нека стойност = цикъл {
                прекъсване i;
            };

            докато x < стойност {
                x += 1;
            }

            x = ако нека Нещо(резултат) = може_би(i) {
                резултат.разопаковане()
            } или {
                12
            };
        }
    }
}
```

## Other languages

- French: [rouille](https://github.com/bnjbvr/rouille)
- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [ржавчина](https://github.com/FluxIndustries/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Spanish: [oxido](https://github.com/fdschonborn/oxido)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)
- Turkish: [pas](https://github.com/ekimb/pas)
- Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
- Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
- Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
- Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
- Romanian: [rugină](https://github.com/aionescu/rugina)
- Czech: [rez](https://github.com/radekvit/rez)
- Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
