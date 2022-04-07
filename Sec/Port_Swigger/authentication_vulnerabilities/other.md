# Vulnerabilidades logins baseados em outros mecanismos

### Mantendo os usuários conectados

Uma funcionalidade é a opção de se manter conectados mesmo depois de fechar a sua sessão.

Essa funcionalidade é normalmente implementada gerando algum tipo de token, que é armazenado em um cookie persistente. Portanto possuir esse cookie permite o usuário passar toda a parte de *logar* nos sitema, é uma boa prática o cookie ser imprevisível. Mas, alguns sites criam os tokens que são formados pela concatenação do usuário e o tempo que foi criado. Algumas vezes até a senha é usada para gerar o token. Esses tipos de tokens são perigosos, pois permitem o atacante criar uma conta e estudar esses cookies e então atacar outras contas.

Mesmo que o atacante não consiga criar a sua própria conta, ainda é possível que eles consigam explorar essa vulnerabilidade. Usando técnicas como XSS, o atacante poderia roubar o cookie do usuário e até descobrir os dados do usuário.

Em casos raros talvez seja possível conseguir a senha do usuário se baseando no cookie, mesmo que o cookie tiver passado por uma função de hash. Listas de hashes podem ser facilmente encontrados online, então não é uma dificuldade descobrir a senha usada para gerar o hash.

### *Resetando* senhas de usuários

Na prática alguns usuários vão esquecer as suas senhas, então é comum ter uma forma de *resetar* sua senha. Como uma autenticação normal de senha não é possível, os sites precisam usar métodos alternativos para saber se é realmente o usuário que está *resetando* a senha. Por essa razão a funcionalidade de resetar senha deve ser bem implementada.

### Enviando senhas por e-mail

Primeiramente os sites não deveriam salvar a senha dos usuários. Ao invés disso, alguns sites geram novas senhas e enviam para o usuário por email.

Enviar senhas persistentes em canais inseguros devem ser evitados. Nesse caso, a segurança se dá caso a senha gerada vai expirar em um período pequeno de tempo, ou a senha é modificada imediatamente. De outra forma, ele é um alvo fácil para o ataque `man-in-the-middle`.

O email normalmente não é considerado seguro, dado que as caixas de inbox são persistentes e não foram feitas para armazenar informações de forma segura. Muitos usuários automaticamente realizam o *sync* no inbox de mútiplos dispositivos por meios de comunitação inseguros.

### Resetar senhas por meio de URL


