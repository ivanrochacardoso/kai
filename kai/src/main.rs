use rdev::{listen, Event, EventType, Key};
use enigo::{Enigo, Keyboard, Settings}; // Enigo 0.2 tem API diferente
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use reqwest::Client;

// use tray_item::TrayItem;

fn main() {
	
    
    // Arc permite compartilhamento entre threads, Mutex garante acesso exclusivo
    let buffer = Arc::new(Mutex::new(String::new()));
    let is_typing = Arc::new(Mutex::new(false));
    
    // Cliente HTTP reutilizável para chamadas da API
    let client = Client::new();

    // Clone do buffer para usar dentro da closure do listener
    let buffer_clone = buffer.clone();
    let client_clone = client.clone();
    let is_typing_clone = is_typing.clone();

    println!("🚀 Espanso IA iniciado!");
    println!("💡 Digite ':pergunta <sua pergunta>/' em qualquer lugar");
    println!("📝 Exemplo: ':pergunta o que é rust?/'");
    println!("⚡ A barra '/' no final dispara a consulta!");
    println!("❌ Para sair, pressione Ctrl+C");

    // O listen é uma função blocking que captura todos os eventos de teclado do sistema
    listen(move |event: Event| {
        // Se a flag 'is_typing' for verdadeira, ignora o evento de tecla.
        if *is_typing_clone.lock().unwrap() {
            return;
        }
        // Só processamos eventos de tecla pressionada (não release)
        if let EventType::KeyPress(key) = event.event_type {
            // Processa diferentes tipos de tecla
            match key {
                Key::Backspace => {
                    // Remove último caractere do buffer quando Backspace é pressionado
                    let mut buf = buffer_clone.lock().unwrap();
                    buf.pop();
                    println!("🔙 Buffer atual: '{}'", buf); // Debug
                }
                Key::Space => {
                    // Adiciona espaço ao buffer
                    let mut buf = buffer_clone.lock().unwrap();
                    buf.push(' ');
                    println!("🔹 Buffer atual: '{}'", buf); // Debug
                }
                Key::Return => {
                    // Enter limpa o buffer (assume que linha foi "enviada")
                    let mut buf = buffer_clone.lock().unwrap();
                    println!("↩️  Enter pressionado, limpando buffer");
                    *buf = String::new();
                }
                // Para todas as outras teclas, tenta converter para caractere
                _ => {
                    if let Some(c) = key_to_char(key) {
                        let mut buf = buffer_clone.lock().unwrap();
                        buf.push(c);
                        println!("📝 Buffer atual: '{}'", buf); // Debug
                    }
                }
            }

            // VERIFICAÇÃO DO COMANDO :pergunta com trigger "/"
            // Clona o buffer para verificar se contém nosso comando
            let buf = buffer_clone.lock().unwrap().clone();
            
            // Processa quando encontra ":pergunta algo/" (termina com barra)
            if buf.starts_with(":pergunta ") && buf.ends_with("/") {
                // Extrai a pergunta removendo ":pergunta " e a "/" final
                let query = buf
                    .trim_start_matches(":pergunta ")
                    .trim_end_matches("/")
                    .to_string();
                
                // Só processa se há conteúdo na pergunta
                if !query.is_empty() {
                    println!("🤔 Pergunta detectada: '{}' (trigger: barra detectada!)", query);

                    // Clones necessários para mover para a thread
                    let client_for_thread = client_clone.clone();
                    let buffer_for_clear = buffer_clone.clone();
                    let is_typing_for_thread = is_typing_clone.clone();

                    // Usa uma thread separada para não bloquear o listener
                    // (o listen precisa ser responsivo para capturar teclas)
                     *is_typing_for_thread.lock().unwrap() = true;
                    thread::spawn(move || {
                        // Cria um runtime tokio só para esta thread
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        
                        rt.block_on(async {
                            println!("🔄 Chamando API...");
                            
                            // Chama a API e obtém resposta
                            let resposta = chama_api_openrouter(&client_for_thread, &query)
                                .await
                                .unwrap_or_else(|e| format!("❌ Erro na API: {}", e));
                            
                            println!("✅ Resposta obtida: {}", resposta);

                            // SIMULAÇÃO DE DIGITAÇÃO
                            // Aguarda um pouco para garantir que o listener processou todas as teclas
                            tokio::time::sleep(Duration::from_millis(100)).await;
                            
                            // Cria instância do Enigo para simular digitação
                            let mut enigo = Enigo::new(&Settings::default())
                                .expect("Falha ao inicializar Enigo");

                            // Apaga o comando digitado (seleciona tudo com Ctrl+A e deleta)
                            // Método mais confiável que pressionar backspace N vezes
                            // println!(" Substituindo texto...");
                            
                            // Seleciona todo o texto da linha atual
                            // enigo.key(enigo::Key::Control, enigo::Direction::Press).unwrap();
                            // enigo.key(enigo::Key::Unicode('a'), enigo::Direction::Press).unwrap();
                            // enigo.key(enigo::Key::Unicode('a'), enigo::Direction::Release).unwrap();
                            // enigo.key(enigo::Key::Control, enigo::Direction::Release).unwrap();
                            
                            // Pequena pausa
                            // thread::sleep(Duration::from_millis(50));

                            // Ativa a trava, digita, e desativa a trava

                            enigo.text(&resposta).unwrap();
                            *is_typing_for_thread.lock().unwrap() = false;

                            // Limpa o buffer para evitar reprocessamento
                            let mut b = buffer_for_clear.lock().unwrap();
                            *b = String::new();
                            
                            println!("✨ Resposta inserida com sucesso!");
                        });
                    });
                }
            }
        }
    }).expect("❌ Erro ao iniciar listener de teclado");
}

/// Mapeia teclas físicas para caracteres
/// Esta função converte as teclas capturadas pelo rdev em caracteres utilizáveis
fn key_to_char(key: Key) -> Option<char> {
    println!("[DEBUG] Key pressed: {:?}", key);
    match key {
        // Letras (sempre minúsculas - maiúsculas requerem detecção de Shift)
        Key::KeyA => Some('a'), Key::KeyB => Some('b'), Key::KeyC => Some('c'), Key::KeyD => Some('d'),
        Key::KeyE => Some('e'), Key::KeyF => Some('f'), Key::KeyG => Some('g'), Key::KeyH => Some('h'),
        Key::KeyI => Some('i'), Key::KeyJ => Some('j'), Key::KeyK => Some('k'), Key::KeyL => Some('l'),
        Key::KeyM => Some('m'), Key::KeyN => Some('n'), Key::KeyO => Some('o'), Key::KeyP => Some('p'),
        Key::KeyQ => Some('q'), Key::KeyR => Some('r'), Key::KeyS => Some('s'), Key::KeyT => Some('t'),
        Key::KeyU => Some('u'), Key::KeyV => Some('v'), Key::KeyW => Some('w'), Key::KeyX => Some('x'),
        Key::KeyY => Some('y'), Key::KeyZ => Some('z'),

        // Números da fileira de cima
        Key::Num0 => Some('0'), Key::Num1 => Some('1'), Key::Num2 => Some('2'), Key::Num3 => Some('3'),
        Key::Num4 => Some('4'), Key::Num5 => Some('5'), Key::Num6 => Some('6'), Key::Num7 => Some('7'),
        Key::Num8 => Some('8'), Key::Num9 => Some('9'),

        // Pontuação - LAYOUT BRASILEIRO ABNT2 (Simplificado)
        Key::SemiColon => Some(':'),      // Mapeado para : como no código original
        Key::Slash | Key::Quote => Some('/'), // Barra (pode estar em Quote dependendo do layout)
        Key::Comma => Some(','),
        Key::Dot => Some('.'),
        
        // Caracteres especiais comuns
        Key::Minus => Some('-'),
        Key::Equal => Some('='),
        Key::LeftBracket => Some('['),
        Key::RightBracket => Some(']'),
        Key::BackSlash => Some('\\'),
        Key::BackQuote => Some('`'),
        
        // Teclas não mapeadas para caracteres, mas que não devem printar erro
        Key::ShiftLeft | Key::ShiftRight | Key::ControlLeft | Key::ControlRight | Key::Alt | Key::AltGr => None,
        Key::MetaLeft | Key::MetaRight | Key::CapsLock | Key::Tab | Key::Escape => None,
        Key::F1 | Key::F2 | Key::F3 | Key::F4 | Key::F5 | Key::F6 | Key::F7 | Key::F8 | Key::F9 | Key::F10 | Key::F11 | Key::F12 => None,

        // Debug: vamos ver qual tecla está sendo capturada
        _ => {
            // Ativa debug para teclas não mapeadas
            println!("🔍 Tecla não mapeada: {:?}", key);
            None
        }
    }
}

/// Chama a API do OpenRouter (ou qualquer LLM)
/// Por enquanto usa mock para economizar créditos durante desenvolvimento
async fn chama_api_openrouter(client: &Client, query: &str) -> Result<String, reqwest::Error> {
    // 🧪 MOCK PARA DESENVOLVIMENTO - sem gastar créditos!
    // Simula tempo real de resposta da API
    tokio::time::sleep(Duration::from_millis(800)).await;
    
    // Respostas mockadas mais realistas baseadas em palavras-chave
    // let resposta = if query.to_lowercase().contains("rust") {
    //     "Rust é uma linguagem de programação systems que foca em segurança, velocidade e concorrência.".to_string()
    // } else if query.to_lowercase().contains("python") {
    //     "Python é uma linguagem interpretada, de alto nível e de propósito geral conhecida por sua sintaxe simples.".to_string()
    // } else if query.to_lowercase().contains("javascript") || query.to_lowercase().contains("js") {
    //     "JavaScript é a linguagem de programação da web, executada tanto no browser quanto no servidor (Node.js).".to_string()
    // } else if query.to_lowercase().contains("como") {
    //     "Aqui está um guia rápido sobre sua pergunta...".to_string()
    // } else if query.to_lowercase().contains("o que é") || query.to_lowercase().contains("what is") {
    //     format!("Essa é uma ótima pergunta sobre {}! Vou explicar de forma simples...", 
    //             query.replace("o que é", "").replace("what is", "").trim())
    // } else {
    //     "Interessante pergunta! Baseado no contexto, posso dizer que...".to_string()
    // };
    
    // Ok(format!("🤖 {}", resposta))
    
    // ⚠️ CONFIGURAÇÃO REAL DA API OPENROUTER (descomente quando quiser usar de verdade)
    
    let api_key = "sk-or-v1-xxxx"; // Sua chave do OpenRouter
    
    let payload = json!({
        "model": "openai/gpt-3.5-turbo", // Modelo barato para testes
        "messages": [
            {
                "role": "user", 
                "content": query
            }
        ],
        "max_tokens": 2000,  // Resposta curta para economizar
        "temperature": 0.7
    });

    println!("📡 Enviando para OpenRouter: {}", query);

    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "http://localhost:3000") // Opcional
        .header("X-Title", "Espanso IA Clone") // Opcional
        .json(&payload)
        .send()
        .await?
        .error_for_status()?; // <-- Tratamento de erro idiomático

    let json_response: serde_json::Value = response.json().await?;
    
    let content = json_response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("❌ Resposta vazia da API")
        .to_string();
    
    Ok(content)
    
}
