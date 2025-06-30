int main() {
    Note tmp(syllablename = "::1", duration = "2/1");
    Bar last;
	Score x(syllablename = ":1 :1 :1 {:1 :2} | :3 :1 :1 {:1 7} | {6 :1} {7 6} {b6 6} {3 5}");
	int a[10] = {49, 46, 119, 97, 118}; // 1.wav
	int b[10] = {50, 46, 119, 97, 118}; // 2.wav
    last.push_note(tmp);
    last.push_note(tmp);
    Bar rep(last);
    Bar torep(syllablename = ":b5 :#5 {:7 :7} :6");
    Score empty;
    x.push_bar(last);
    empty.push_bar(rep);
    Score new_empty(empty);
    x.append(new_empty);
    x.set_score_bpm(150);
    x.replace_bar(2, torep);
	x.sing(a, b, 44100, 16, 2);
    return 0;
}
