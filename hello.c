int main() {
	Score x(syllablename = ":1 :1 :5 :5 | :6 :6 :5 - | :4 :4 :3 :3 | :2 :2 :1 - ");
    Score x1(syllablename = "1 - - -  | 4 - - - | 5$ - 7$ -  | 2 5 1 - ");
    Score x2(syllablename = "{::1 :5} {::1 ::3} {::5 ::1} {::3 ::5} | {::6 ::4} {:::1 ::6} {::5 ::3} {::1 ::3} | {::4 :5} {::2 ::4} {:7 :5} {:7 ::3} | {::2 :5} {:7 ::2} ::1 - ");
	int a[10] = {49, 46, 119, 97, 118}; // 1.wav
	int b[10] = {50, 46, 119, 97, 118}; // 2.wav
	int c[10] = {51, 46, 119, 97, 118}; // 3.wav
	int d[10] = {52, 46, 119, 97, 118}; // 4.wav
    x.inc_score_pitch(-12);
    x1.inc_score_pitch(-12);
    x2.inc_score_pitch(-12);
	x.sing(a, b, 44100, 16, 2);
    x1.sing(a, c, 44100, 16, 2);
    x2.sing(a, d, 44100, 16, 2);
    Track t1(b);
    Track t2(c);
    Track t3(d);
    t1.stack(t2);
    t1.stack(t3);
    return 0;
}
