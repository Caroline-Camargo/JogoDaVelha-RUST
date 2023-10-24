// Importando bibliotecas
use ggez; 
use ggez::event; 
use ggez::graphics; 
use ggez::{Context, GameResult}; 
use nalgebra as na;
use rand::Rng;


// Constantes que definem o tamanho dos quadrados do jogo da velha e o espaço entre eles.
const LARGURA_QUADRADO: f32 = 80.0;
const ESPACO_ENTRE_QUADRADOS: f32 = 10.0;

// Definição da estrutura de dados que representa o estado do jogo.
struct EstadoJogo {
    posicao_bloco: Vec<Vec<na::Point2<f32>>>, // Matriz que armazena as posições dos blocos no jogo da velha (pontos bidimensionais: biblioteca nalgebra)
    conteudo_bloco: Vec<Vec<Option<String>>>, // Matriz com a descrição do conteúdo dos quadrados (X, O ou vazio).
    jogador_atual: String, // Jogador atual (X ou O).
    terminou: bool,
    jogador_vencedor: Option<String>,
    mensagem_vitoria: Option<String>,
}

impl EstadoJogo {
    // Construtor para criar um novo estado de jogo.
    fn novo() -> GameResult<EstadoJogo> {
        // Inicializa as posições onde os blocos devem começar.
        let x = 100.0;
        let y = 100.0;

        let mut posicao_bloco = Vec::with_capacity(3);  // Aloca 3 posições para o vetor
        let mut conteudo_bloco = Vec::with_capacity(3);

        for linha in 0..3 { // Percorre as linhas
            let mut linha_tempposicao = Vec::with_capacity(3);
            let mut linha_tempconteudo = Vec::with_capacity(3);

            let y_bloco = y + linha as f32 * (LARGURA_QUADRADO + ESPACO_ENTRE_QUADRADOS); // Cordenada y do quadrado
            for coluna in 0..3 { // Percorre as colunas
                let x_bloco = x + coluna as f32 * (LARGURA_QUADRADO + ESPACO_ENTRE_QUADRADOS); // Cordenada x do quadrado
                
                linha_tempposicao.push(na::Point2::new(x_bloco, y_bloco));
                linha_tempconteudo.push(None);
            }

            posicao_bloco.push(linha_tempposicao);
            conteudo_bloco.push(linha_tempconteudo);
        }

        let terminou = false;

        // Retorna o estado inicializado com o jogador atual como "X".
        Ok(EstadoJogo {
            posicao_bloco,
            conteudo_bloco,
            jogador_atual: "X".to_string(),
            terminou,
            jogador_vencedor: Some("".to_string()),
            mensagem_vitoria: Some("".to_string()),
        })
    }

    // Verifica se um clique ocorreu dentro de um quadrado válido e retorna sua posição.
    fn verificar_clique_quadrado(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        for linha in 0..3 {
            for coluna in 0..3 {
                let posicao_bloco = self.posicao_bloco[linha][coluna]; // Coordenadas do centro do bloco
                let dentro_x = x >= posicao_bloco.x - LARGURA_QUADRADO * 0.5
                    && x <= posicao_bloco.x + LARGURA_QUADRADO * 0.5; // Verifica coordenada mínima e máxima na horizontal do bloco
                let dentro_y = y >= posicao_bloco.y - LARGURA_QUADRADO * 0.5
                    && y <= posicao_bloco.y + LARGURA_QUADRADO * 0.5; // Verifica coordenada mínima e máxima na vertical do bloco
                
                if dentro_x && dentro_y && self.conteudo_bloco[linha][coluna].is_none() {
                    return Some((linha, coluna));
                }
            }
        }
        None
    }

    // Verifica se o jogador atual venceu 
    fn verificar_vitoria(&self, linha: usize, coluna: usize) -> bool {
        // Obtém o jogador atual com base na última marcação na posição especificada.
        let jogador_atual = match &self.conteudo_bloco[linha][coluna] {
            Some(jogador) => jogador,
            None => return false, // Retorna false se não houver marcação no bloco atual.
        };
        
        // Verifica se o jogador atual venceu na linha onde a última marcação foi feita.
        for i in 0..3 {
            if self.conteudo_bloco[linha][i].as_ref() != Some(jogador_atual) {
                break; // Se uma marcação não corresponder, sai do loop.
            }
            if i == 2 {
                return true; // Se todas as marcações na linha corresponderem, retorna true.
            }
        }
    
        // Verifica se o jogador atual venceu na coluna onde a última marcação foi feita.
        for i in 0..3 {
            if self.conteudo_bloco[i][coluna].as_ref() != Some(jogador_atual) {
                break; // Se uma marcação não corresponder, sai do loop.
            }
            if i == 2 {
                return true; // Se todas as marcações na coluna corresponderem, retorna true.
            }
        }
    
        // Verifica se o jogador atual venceu na diagonal principal.
        if linha == coluna {
            for i in 0..3 {
                if self.conteudo_bloco[i][i].as_ref() != Some(jogador_atual) {
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
                if self.conteudo_bloco[i][2 - i].as_ref() != Some(jogador_atual) {
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
    

    // Verifica se o jogo terminou em empate (todos blocos devem ter sido preenchidos).
    fn verificar_empate(&self) -> bool {
        for linha in &self.conteudo_bloco {
            for marcacao in linha {
                if marcacao.is_none() {
                    return false;
                }
            }
        }
        true
    }

    fn verificar_vitoria_iminente(&mut self, jogador: &str) -> Option<(usize, usize)> {
        // Verificar se o jogador pode ganhar na próxima jogada.
        for linha in 0..3 {
            for coluna in 0..3 {
                if self.conteudo_bloco[linha][coluna].is_none() {
                    self.conteudo_bloco[linha][coluna] = Some(jogador.to_string());
                    if self.verificar_vitoria(linha, coluna) {
                        self.conteudo_bloco[linha][coluna] = None;
                        return Some((linha, coluna));
                    }
                    self.conteudo_bloco[linha][coluna] = None;
                }
            }
        }
        None
    }

    // Atualiza o estado do jogo para reiniciar o jogo vazio.
    fn reiniciar(&mut self) {
        for linha in 0..3 {
            for coluna in 0..3 {
                self.conteudo_bloco[linha][coluna] = None;
            }
        }
        self.jogador_atual = "X".to_string();

        // Reinicia a variável "terminou" para falso.
        self.terminou = false;
        self.jogador_vencedor = Some("".to_string());
        self.mensagem_vitoria = Some("".to_string());
    }
}


impl event::EventHandler for EstadoJogo {
    // Método chamado a cada atualização do jogo.
    fn update(&mut self, _: &mut Context) -> GameResult { // Verifica todas posições do tabuleiro para saber se algum jogador ganhou
        if self.terminou {
            // O jogo já terminou, não faz nada.
            return Ok(());
        }

        for linha in 0..3 {
            for coluna in 0..3 {
                if self.verificar_vitoria(linha, coluna) {
                    self.jogador_vencedor = Some("O".to_string());
                    self.mensagem_vitoria = Some(format!("Jogador O venceu!"));
                    self.terminou = true;
                    return Ok(());
                }
            }
        }

        // Se o jogo terminou em empate, defina a variável "terminou" como verdadeira.
        if self.verificar_empate() {
            self.mensagem_vitoria = Some("O jogo terminou em empate!".to_string());
            self.terminou = true;
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
                let posicao = self.posicao_bloco[linha][coluna];
                let retangulo_quadrado = graphics::Rect::new(
                    LARGURA_QUADRADO * -0.5,
                    LARGURA_QUADRADO * -0.5,
                    LARGURA_QUADRADO,
                    LARGURA_QUADRADO,
                );
                
                // Cria uma malha (retângulo) representando o quadrado com a cor definida.
                let malha_quadrado = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    retangulo_quadrado,
                    cor_quadrado,
                )?;
    
                let mut parametro_desenho_quadrado = graphics::DrawParam::default();
                parametro_desenho_quadrado.dest.x = posicao.x;
                parametro_desenho_quadrado.dest.y = posicao.y;

    
                // Desenha o quadrado na posição atual.
                graphics::draw(ctx, &malha_quadrado, parametro_desenho_quadrado)?;
    
                // Desenha as marcações (X ou O) nos quadrados se existirem.
                if let Some(marcacao) = &self.conteudo_bloco[linha][coluna] {
                    // Cria um texto com a marcação.
                    let texto = graphics::Text::new(marcacao.clone());
                    
                    // Calcula a posição para a marcação (levemente deslocada do centro do quadrado).
                    let posicao_marcacao = na::Point2::new(posicao.x - 10.0, posicao.y - 10.0);
        
                    let mut parametro_desenho_marcacao = graphics::DrawParam::default();
                    parametro_desenho_marcacao.dest.x = posicao_marcacao.x;
                    parametro_desenho_marcacao.dest.y = posicao_marcacao.y;
    
                    // Desenha a marcação (X ou O) na posição calculada.
                    graphics::draw(ctx, &texto, parametro_desenho_marcacao)?;
                }
            }
        }
    
        // Desenha o título na tela.
        let titulo_texto = graphics::Text::new("JOGO DA VELHA");
        let titulo_posicao = na::Point2::new(140.0, 20.0);
    
        let mut parametro_desenho_titulo = graphics::DrawParam::default();
        parametro_desenho_titulo.dest.x = titulo_posicao.x;
        parametro_desenho_titulo.dest.y = titulo_posicao.y;
        parametro_desenho_titulo.color = graphics::BLACK;
    
        // Desenha o título na posição especificada.
        graphics::draw(ctx, &titulo_texto, parametro_desenho_titulo)?;
        
        if let Some(mensagem) = &self.mensagem_vitoria {
            // Exibe a mensagem de vitória na tela.
            let mensagem_texto = graphics::Text::new(mensagem.clone());
            let mensagem_posicao = na::Point2::new(60.0, 330.0); // Posição da mensagem de vitória.

            let mut parametro_desenho_mensagem = graphics::DrawParam::default();
            parametro_desenho_mensagem.dest.x = mensagem_posicao.x;
            parametro_desenho_mensagem.dest.y = mensagem_posicao.y;
            parametro_desenho_mensagem.color = graphics::BLACK;

            graphics::draw(ctx, &mensagem_texto, parametro_desenho_mensagem)?;
        }

        // Apresenta o que foi desenhado na tela.
        graphics::present(ctx)?;
        Ok(())
    }
    

    // Esta função lida com eventos de clique do mouse no jogo da velha.
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: event::MouseButton, x: f32, y: f32) {    
        if button == event::MouseButton::Left {
            if self.terminou {
                self.reiniciar();
                return;
            }
            // Chama a função verificar clique para determinar se o clique ocorreu em um quadrado válido.
            if let Some((mut linha, mut coluna)) = self.verificar_clique_quadrado(x, y) {
                // Verifica se o quadrado clicado está vazio (sem marcação).
                if self.conteudo_bloco[linha][coluna].is_none() {
                    // Marca o quadrado com o símbolo do jogador atual (X ou O).
                    self.conteudo_bloco[linha][coluna] = Some("X".to_string());
    
                    // Verifica se o jogador atual venceu após fazer a marcação.
                    if self.verificar_vitoria(linha, coluna) {
                        // Se o jogador venceu, imprime uma mensagem de vitória.
                        self.jogador_vencedor = Some(self.jogador_atual.clone());
                        self.mensagem_vitoria = Some(format!("Jogador {} venceu!", self.jogador_atual));
                        self.terminou = true;
                        
                    } else {
                        // Verifica se o jogo terminou em empate.
                        let empate = self.verificar_empate();
    
                        if empate {
                            // Se o jogo terminou em empate, imprime uma mensagem de empate.
                            self.mensagem_vitoria = Some("O jogo terminou em empate!".to_string());
                            self.terminou = true;
                    
                        } else { //Jogo não terminou
                            // Vez do computador jogar.
                            self.jogador_atual = "O".to_string();

                            if let Some((linha, coluna)) = self.verificar_vitoria_iminente("O") {
                                self.conteudo_bloco[linha][coluna] = Some("O".to_string());
                            } else if let Some((linha, coluna)) = self.verificar_vitoria_iminente("X") {
                                self.conteudo_bloco[linha][coluna] = Some("O".to_string());
                            } else {
                                // Jogada aleatória do Computador
                                let mut rng = rand::thread_rng();
                                loop {
                                    linha = rng.gen_range(0..3);
                                    coluna = rng.gen_range(0..3);

                                    if self.conteudo_bloco[linha][coluna].is_none() {
                                        self.conteudo_bloco[linha][coluna] = Some("O".to_string());
                                        break;
                                    }
                                }
                            }

                            // Verifica se o computador venceu ou empatou após fazer a marcação.
                            let vitoria = self.verificar_vitoria(linha, coluna);
                            if vitoria {
                                self.jogador_vencedor = Some(self.jogador_atual.clone());
                                self.mensagem_vitoria = Some(format!("Jogador {} venceu!", self.jogador_atual));
                                self.terminou = true;
                            } else {
                                let empate = self.verificar_empate();
                                if empate {
                                    self.mensagem_vitoria = Some("O jogo terminou em empate!".to_string());
                                    self.terminou = true;
                                } else {
                                    self.jogador_atual = "X".to_string();
                                }
                            }
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
    let altura_tela = 3.0 * (LARGURA_QUADRADO + ESPACO_ENTRE_QUADRADOS) + 100.0;

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