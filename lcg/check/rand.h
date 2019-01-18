/* Generador Congruencial Lineal Multiplicatiu de M�dul Primer "GCLMMP"
 * Z[i] = (630360016*z[i-1])(mod(pow(2,31)-1)). Soportant diferents segments (100)
 * amb llavors separades. �s: (Tres funcions)
 * 
 * 1. Per obtenir  el pr�xim nombre aleatori U(0,1) del segment "stream", executar
 * 	"u = rand(stream);" a on "rand" �s una funci� i la variable que retorna "u" 
 * 	ser� el pr�xim nombre aleatori.
 * 2. Per definir la llavor del segment "stream" amb un valor "zset" desitjat, executar
 * 	"randst(zset, stream);" a on "randst" �s una funci� nul.la (no retorna cap valor)
 * 	i "zset" deu ser una bona llavor, un nombre entre 1 i 2147483646. El codi cont� 
 * 	llavors inicials per a tots els posibles segments (100).
 * 3. Per obtenir l'actual enter en la seq�encia que ha estat iniciada pel segment "stream" 
 * 	a la variable "zget", executar "zget = randgt(stream);". */


/* Definici� de les constants. */
#define	MODLUS 2147483647
#define	MULT1 24112
#define	MULT2 26143

/* Defineix les llavors inicials per a 100 segments. */
static long zrng[]=
{	0,
	1973272912,	281629770,	20006270,	1280689831,	2096730329,	1933576050,
	913566091,	246780520,	1363774876,	604901985,	1511192140,	1259851944,
	824064364,	150493284,	242708531,	75253171,	1964472944,	1202299975,
	233217322,	1911216000,	726370533,	403498145,	993232223,	1103205531,
	762430696,	1922803170,	1385516923,	76271663,	413682397,	726466604,
	336157058,	1432650381,	1120463904,	595778810,	877722890,	1046574445,
	68911991,	2088367019,	748545416,	622401386,	2122378830,	640690903,
	1774806513,	2132545692,	2079249579,	78130110,	852776735,	1187867272,
	1351423507,	1645973084,	1997049139,	922510994,	2045512870,	898585771,
	243649545,	1004818771,	773686062,	403188473,	372279877,	1901633463,
	498067494,	2087759558,	493157915,	597104727,	1530940798,	1814496276,
	536444882,	1663153658,	855503735,	67784357,	1432404475,	619691088,
	119025595,	880802310,	176192644,	1116780070,	277854671,	1366580350,
	1142483975,	2026948561,	1053920743,	786262391,	1792203830,	1494667770,
	1923011392,	1433700034,	1244184613,	1147297105,	539712780,	1545929719,
	190641742,	1645390429,	264907697,	620389253,	1502074852,	927711160,
	364849192,	2049576050,	638580085,	547070247};


/* Genera el pr�xim nombre aleatori. */
double rand(int stream)
{
	long zi, lowprd, hi31;
	zi	=	zrng[stream];	
	lowprd	=	(zi & 65535) * MULT1;
	hi31	=	(zi >> 16) * MULT1 + (lowprd >> 16);
	zi	=	((lowprd & 65535) - MODLUS)+
			((hi31 & 32767) << 16) + (hi31 >> 15);
	if (zi < 0) zi += MODLUS;
	lowprd	=	(zi & 65535) * MULT2;
	hi31	=	(zi >> 16) * MULT2 + (lowprd >> 16);
	zi	=	((lowprd & 65535) - MODLUS)+
			((hi31 & 32767) << 16) + (hi31 >> 15);
	if (zi < 0) zi += MODLUS;
	zrng[stream] = zi;
	return ((zi >> 7 | 1) + 1)/ 16777216.0;
}

/* Defineix l'actual "zrng" per al segment "stream". */
void randst (long zset, int stream)
{
	zrng[stream] = zset;
}

/* Retorna l'actual "zrng" del segment "stream". */
long randgt (int stream)
{
	return zrng[stream];
}

/* Les 3 declaracions seg�ents son per usar el generador de nombres aleatoris,
 * "rand" i les funcions asociades "randst" i "randgt" per la gesti� de la llavor.
 * Aquest fitxer (anomenat "rand.h") hauria d'estar incluit en qualsevol programa 
 * que faci servir aquestes funcions mitjan�an l'instrucci� "#include "rand.h""
 * abans de refenciar-les. */
double rand (int stream);
void randst (long zset, int stream);
long randgt (int stream);

