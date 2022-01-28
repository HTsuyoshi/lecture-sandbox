CXXFLAGS = -Wall -g	# Opções do compilador: todos warning e debug info

PROG = main
FONTES = main.cpp matematica.cpp
OBJETOS = $(FONTES:.cpp=.o)

$(PROG): $(OBJETOS)
	g++ $(CXXFLAGS) $(OBJETOS) -o $@

clean:
	-@ rm -f $(OBJETOS) $(PROG)
