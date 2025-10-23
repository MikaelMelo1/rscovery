# 🧩 Rodando o Projeto RSCOVERY

## 📥 1. Clonar o repositório

```bash
git clone https://github.com/YuriRDev/rscovery
cd rscovery
```

---

## ⚙️ 2. Instalar dependências do sistema

Certifique-se de ter o **Git**, **Node.js/NPM** e bibliotecas necessárias para o Tauri instaladas:

```bash
sudo apt update
sudo apt install -y build-essential git libwebkit2gtk-4.0-dev libgtk-3-dev \
pkg-config libglib2.0-dev curl
```

> 💡 Caso esteja no **Ubuntu 22.04**, pode ser necessário usar `libwebkit2gtk-4.1-dev`:
>
> ```bash
> sudo apt install -y libwebkit2gtk-4.1-dev
> ```

---

## 🧰 3. Instalar Node.js e NPM

Se ainda não tiver o Node.js e o npm instalados:

```bash
sudo apt install -y nodejs npm
```

Verifique se foi instalado corretamente:

```bash
node -v
npm -v
```

---

## 🦀 4. Instalar o Rust

Baixe e instale o Rust com o `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Selecione a opção **1 (default)** quando solicitado.

Após a instalação, recarregue o ambiente:

```bash
source $HOME/.cargo/env
```

Verifique se o Rust está instalado:

```bash
rustc --version
cargo --version
```

---

## 🚀 5. Instalar dependências do projeto

Dentro da pasta do projeto:

```bash
npm install
npm install -g yarn
```

---

## 🛠️ 6. Instalar o Tauri CLI

Você pode escolher **uma das duas opções** abaixo:

**Via Cargo (Rust):**

```bash
cargo install tauri-cli
```

**Ou via NPM:**

```bash
npm install --save-dev @tauri-apps/cli@latest
```

---

## ▶️ 7. Rodar o projeto

Após todas as dependências instaladas:

```bash
npm run tauri dev
```

---

## ✅ 8. (Opcional) Corrigir permissões

Se ocorrerem erros de permissão ao usar `cargo` ou `npm`, rode:

```bash
sudo chown -R $USER:$USER ~/.cargo ~/.npm
```

---

### 🔧 Resumo rápido dos comandos

```bash
sudo apt update
sudo apt install -y build-essential git libwebkit2gtk-4.0-dev libgtk-3-dev pkg-config libglib2.0-dev curl nodejs npm
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
git clone https://github.com/YuriRDev/rscovery
cd rscovery
npm install
npm install -g yarn
cargo install tauri-cli
npm run tauri dev
```

---
