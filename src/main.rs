// Importando bibliotecas
use ggez; 
use ggez::event; 
use ggez::graphics; 
use ggez::{Context, GameResult}; 
use ggez::nalgebra as na; 

// Constantes que definem o tamanho dos quadrados do jogo da velha e o espaço entre eles.
const LARGURA_QUADRADO: f32 = 80.0;
const ALTURA_QUADRADO: f32 = 80.0;
const ESPACO_ENTRE_QUADRADOS: f32 = 10.0;

// Definição da estrutura de dados que representa o estado do jogo.
struct EstadoJogo {
    posicoes_quadrados: Vec<Vec<na::Point2<f32>>>, // Matriz que armazena as posições dos quadrados no jogo da velha (pontos bidimensionais: biblioteca nalgebra)
    marcacoes_quadrados: Vec<Vec<Option<String>>>, // Matriz com a descrição do conteúdo dos quadrados (X, O ou vazio).
    jogador_atual: String, // Jogador atual (X ou O).
}

impl EstadoJogo {
    // Construtor para criar um novo estado de jogo.
    fn novo() -> GameResult<EstadoJogo> {
        // Inicializa as posições dos quadrados e as marcações como vazias.
        let x = 100.0;
        let y = 100.0;

        let mut posicoes_quadrados = Vec::with_capacity(3);  // Aloca 3 posições para o vetor
        let mut marcacoes_quadrados = Vec::with_capacity(3);

        for linha in 0..3 { // Percorre as linhas
            let mut linha_quadrados = Vec::with_capacity(3);
            let mut linha_marcacoes = Vec::with_capacity(3);

            for coluna in 0..3 { // Percorre as colunas
                let x_quadrado = x + coluna as f32 * (LARGURA_QUADRADO + ESPACO_ENTRE_QUADRADOS);
                let y_quadrado = y + linha as f32 * (ALTURA_QUADRADO + ESPACO_ENTRE_QUADRADOS);

                linha_quadrados.push(na::Point2::new(x_quadrado, y_quadrado));
                linha_marcacoes.push(None);
            }

            posicoes_quadrados.push(linha_quadrados);
            marcacoes_quadrados.push(linha_marcacoes);
        }

        // Retorna o estado inicializado com o jogador atual como "X".
        Ok(EstadoJogo {
            posicoes_quadrados,
            marcacoes_quadrados,
            jogador_atual: "X".to_string(),
        })
    }

    // Verifica se um clique ocorreu dentro de um quadrado válido e retorna sua posição.
    fn verificar_clique_quadrado(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        for linha in 0..3 {
            for coluna in 0..3 {
                let posicao_quadrado = self.posicoes_quadrados[linha][coluna];
                let dentro_x = x >= posicao_quadrado.x - LARGURA_QUADRADO * 0.5
                    && x <= posicao_quadrado.x + LARGURA_QUADRADO * 0.5;
                let dentro_y = y >= posicao_quadrado.y - ALTURA_QUADRADO * 0.5
                    && y <= posicao_quadrado.y + ALTURA_QUADRADO * 0.5;
                
                if dentro_x && dentro_y && self.marcacoes_quadrados[linha][coluna].is_none() {
                    return Some((linha, coluna));
                }
            }
        }
        None
    }

    // Verifica se o jogador atual venceu com base na última marcação feita.
    fn verificar_vitoria(&self, linha: usize, coluna: usize) -> bool {
        // Obtém o jogador atual com base na última marcação na posição especificada.
        let jogador_atual = match &self.marcacoes_quadrados[linha][coluna] {
            Some(jogador) => jogador,
            None => return false, // Retorna false se não houver marcação.
        };
    
        // Verifica se o jogador atual venceu na linha onde a última marcação foi feita.
        for i in 0..3 {
            if self.marcacoes_quadrados[linha][i].as_ref() != Some(jogador_atual) {
                break; // Se uma marcação não corresponder, sai do loop.
            }
            if i == 2 {
                return true; // Se todas as marcações na linha corresponderem, retorna true.
            }
        }
    
        // Verifica se o jogador atual venceu na coluna onde a última marcação foi feita.
        for i in 0..3 {
            if self.marcacoes_quadrados[i][coluna].as_ref() != Some(jogador_atual) {
                break; // Se uma marcação não corresponder, sai do loop.
            }
            if i == 2 {
                return true; // Se todas as marcações na coluna corresponderem, retorna true.
            }
        }
    
        // Verifica se o jogador atual venceu na diagonal principal.
        if linha == coluna {
            for i in 0..3 {
                if self.marcacoes_quadrados[i][i].as_ref() != Some(jogador_atual) {
                    break; // Se uma marcação não corresponder, sai do loop.
                }
                if i == 2 {
                    return true; // Se todas as marcações na diagonal principal corresponderem, retorna true.
                }
            }
        }
    
        // Verifica se o jogador atual venceu na diagonal secundária.
        if linha + coluna == 2 {
            for i in 0..3 {
                if self.marcacoes_quadrados[i][2 - i].as_ref() != Some(jogador_atual) {
                    break; // Se uma marcação não corresponder, sai do loop.
                }
                if i == 2 {
                    return true; // Se todas as marcações na diagonal secundária corresponderem, retorna true.
                }
            }
        }
    
        // Se nenhum dos casos acima for verdadeiro, retorna false, indicando que o jogador atual não venceu.
        false
    }
    

    // Verifica se o jogo terminou em empate.
    fn verificar_empate(&self) -> bool {
        for linha in &self.marcacoes_quadrados {
            for marcacao in linha {
                if marcacao.is_none() {
                    return false;
                }
            }
        }
        true
    }

    // Reinicia o jogo para um novo jogo vazio.
    fn reiniciar(&mut self) {
        for linha in 0..3 {
            for coluna in 0..3 {
                self.marcacoes_quadrados[linha][coluna] = None;
            }
        }
        self.jogador_atual = "X".to_string();
    }
}

impl event::EventHandler for EstadoJogo {
    // Método chamado a cada atualização do jogo.
    fn update(&mut self, _: &mut Context) -> GameResult { // Verifica todas posições do tabuleiro para saber se algum jogador ganhou
        for linha in 0..3 {
            for coluna in 0..3 {
                if self.verificar_vitoria(linha, coluna) {
                    self.reiniciar();
                }
            }
        }
        Ok(())
    }

    // Método chamado para desenhar o estado atual do jogo na tela.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {    
        // Limpa a tela com a cor branca.
        graphics::clear(ctx, graphics::WHITE);
    
        // Define a cor dos quadrados como um tom de vermelho/rosa.
        let cor_quadrado = graphics::Color::new(1.0, 0.13, 0.32, 1.0);
    
        // Itera sobre os quadrados do tabuleiro e desenha cada um.
        for linha in 0..3 {
            for coluna in 0..3 {
                let posicao = self.posicoes_quadrados[linha][coluna];
                let retangulo_quadrado = graphics::Rect::new(
                    -LARGURA_QUADRADO * 0.5,
                    -ALTURA_QUADRADO * 0.5,
                    LARGURA_QUADRADO,
                    ALTURA_QUADRADO,
                );
                
                // Cria uma malha (retângulo) representando o quadrado com a cor definida.
                let malha_quadrado = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    retangulo_quadrado,
                    cor_quadrado,
                )?;
    
                let mut parametro_desenho_quadrado = graphics::DrawParam::default();
                parametro_desenho_quadrado.dest = posicao.into();
    
                // Desenha o quadrado na posição atual.
                graphics::draw(ctx, &malha_quadrado, parametro_desenho_quadrado)?;
    
                // Desenha as marcações (X ou O) nos quadrados se existirem.
                if let Some(marcacao) = &self.marcacoes_quadrados[linha][coluna] {
                    // Cria um texto com a marcação.
                    let marcacao_texto = graphics::Text::new(marcacao.clone());
                    
                    // Calcula a posição para a marcação (levemente deslocada do centro do quadrado).
                    let posicao_marcacao = na::Point2::new(posicao.x - 10.0, posicao.y - 10.0);
    
                    // Define a cor das marcações como branco.
                    let cor_marcacao = graphics::Color::new(1.0, 1.0, 1.0, 1.0);
    
                    let mut parametro_desenho_marcacao = graphics::DrawParam::default();
                    parametro_desenho_marcacao.dest = posicao_marcacao.into();
                    parametro_desenho_marcacao.color = cor_marcacao;
    
                    // Desenha a marcação (X ou O) na posição calculada.
                    graphics::draw(ctx, &marcacao_texto, parametro_desenho_marcacao)?;
                }
            }
        }
    
        // Desenha o título na tela.
        let titulo_texto = graphics::Text::new("JOGO DA VELHA");
        let titulo_posicao = na::Point2::new(140.0, 20.0);
    
        let mut parametro_desenho_titulo = graphics::DrawParam::default();
        parametro_desenho_titulo.dest = titulo_posicao.into();
        parametro_desenho_titulo.color = graphics::BLACK;
    
        // Desenha o título na posição especificada.
        graphics::draw(ctx, &titulo_texto, parametro_desenho_titulo)?;
    
        // Apresenta o que foi desenhado na tela.
        graphics::present(ctx)?;
        Ok(())
    }
    

    // Esta função lida com eventos de clique do mouse no jogo da velha.
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: event::MouseButton, x: f32, y: f32) {    
        // Verifica se o botão pressionado foi o botão esquerdo do mouse.
        if button == event::MouseButton::Left {
            // Chama a função verificar clique para determinar se o clique ocorreu em um quadrado válido.
            if let Some((linha, coluna)) = self.verificar_clique_quadrado(x, y) {
                // Verifica se o quadrado clicado está vazio (sem marcação).
                if self.marcacoes_quadrados[linha][coluna].is_none() {
                    // Marca o quadrado com o símbolo do jogador atual (X ou O).
                    self.marcacoes_quadrados[linha][coluna] = Some(self.jogador_atual.clone());
    
                    // Verifica se o jogador atual venceu após fazer a marcação.
                    if self.verificar_vitoria(linha, coluna) {
                        // Se o jogador venceu, imprime uma mensagem de vitória.
                        println!("Jogador {} venceu!", self.jogador_atual);
    
                        // Reinicia o jogo para um novo jogo vazio.
                        self.reiniciar();
                    } else {
                        // Verifica se o jogo terminou em empate.
                        let empate = self.verificar_empate();
    
                        if empate {
                            // Se o jogo terminou em empate, imprime uma mensagem de empate.
                            println!("O jogo terminou em empate!");
    
                            // Reinicia o jogo para um novo jogo vazio.
                            self.reiniciar();
                        } else {
                            // Alterna o jogador atual entre X e O.
                            self.jogador_atual = if self.jogador_atual == "X" { "O".to_string() } else { "X".to_string() };
                        }
                    }
                }
            }
        }
    }    
}

fn main() -> GameResult {
    // Calcula a largura e altura da tela com base nas constantes.
    let largura_tela = 3.0 * (LARGURA_QUADRADO + ESPACO_ENTRE_QUADRADOS) + 100.0;
    let altura_tela = 3.0 * (ALTURA_QUADRADO + ESPACO_ENTRE_QUADRADOS) + 100.0;

    // Configura o contexto do jogo com um título e dimensões.
    let cb = ggez::ContextBuilder::new("jogo_da_velha", "Caroline")
        .window_setup(ggez::conf::WindowSetup::default().title("Jogo da Velha"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(largura_tela, altura_tela));

    // Inicializa o contexto e o loop de eventos do jogo.
    let (mut ctx, mut event_loop) = cb.build()?;
    graphics::set_window_title(&ctx, "Jogo da Velha");

    // Inicializa o estado do jogo.
    let mut estado = EstadoJogo::novo()?;

    // Inicia o loop de eventos para executar o jogo.
    event::run(&mut ctx, &mut event_loop, &mut estado)
}
