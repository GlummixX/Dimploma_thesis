## Návod na spuštění ukázek
Návod předpokládá, že máte nainstalovaný jazyk rust a potřebné nástroje. Pokud ne, najdete návod na instalaci [zde](https://www.rust-lang.org/tools/install). <br>
Příkazový řadek musí být v adresáři examples, nikoliv v kořenovém adresáři repozitáře.

Pomocí následujícího příkazu můžete zkompilovat spustit libovolnou ukázku:
```bash
cargo run --bin <název_složky_ukázky>
```
Například: 
```bash
cargo run --bin p01_basic_triangle
```

Pro maximální výkon je potřeba spustit ukázku v řežimu release pomocí parametru `-r`: <br>
*Kompilace v režimu release může trvat déle v závislosti na HW.*
```bash
cargo run -r --bin <název_složky_ukázky>
```

Pokud chcete ukázku pouze zkompilovat bez spuštění, použijte příkaz `build` v kombinaci s parametrem `-r` pro optimalizovaný release:
```bash
cargo build -r --bin <název_složky_ukázky>
```
Výsledný binární soubor bude umístěn do složky `examples/target/release`.
Pokud byl soubor kompilován bez parametru `-r`, bude umístěn do složky `examples/target/debug`.