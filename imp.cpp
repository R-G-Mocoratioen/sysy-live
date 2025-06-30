#include <bits/stdc++.h>
using namespace std;

extern "C" {
struct Note {
	const int rest_or_tie; // rest 1 tie 2
	const int half;        // do+几个半音
	const double duration_beat;
};

// 所有的 note/bar 等在 C 中都用 int 编号表示

// 所有的 note 都存在这里，注意 note 是 immutable 的
// 也就是说，不能修改已经存在的 note 的内容，只能新建一个 note
vector<Note> notes;

int newnote_rest(int rest, int fz, int fm) {
	int id = notes.size();
	notes.push_back({rest, 0, 1.0 * fz / fm});
	return id;
}

int newnote(int half, int fz, int fm) {
	int id = notes.size();
	notes.push_back({0, half, 1.0 * fz / fm});
	return id;
}

struct Bar {
	vector<int> notes;
	double tonality; // 默认是 C4
	int bpm;
	Bar() {
		notes.clear();
		tonality = 261.626;
		bpm = 100;
	}
};

vector<Bar> bars;

int newbar() {
	int id = bars.size();
	bars.push_back(Bar());
	return id;
}

void bar_copy(int x, int y) { bars[x] = bars[y]; }

void bar_settonality(int x, int half) { bars[x].tonality = 261.626 * pow(2, half / 12.0); }

void bar_setbpm(int x, int bpm) { bars[x].bpm = bpm; }

void bar_push(int x, int note) {
	bars[x].notes.push_back(note);
	cout << "add note with length " << notes[note].duration_beat << endl;
}

struct Score {
	vector<Bar> bars;
	double tonality;
	int bpm;
	Score() {
		bars.clear();
		tonality = 261.626;
		bpm = 100;
	}
};

vector<Score> scores;

int newscore() {
	int id = scores.size();
	scores.push_back(Score());
	return id;
}

void score_copy(int x, int y) { scores[x] = scores[y]; }

void score_push(int x, int y) { scores[x].bars.push_back(bars[y]); }

void score_append(int x, int y) {
	scores[x].bars.insert(scores[x].bars.end(), scores[y].bars.begin(), scores[y].bars.end());
}

void score_replace(int x, int k, int id) { scores[x].bars[k] = bars[id]; }

void score_settonality(int x, int half) { scores[x].tonality = 261.626 * pow(2, half / 12.0); }

void score_setbpm(int x, int bpm) {
	for (int i = 0; i < scores[x].bars.size(); i++) {
		scores[x].bars[i].bpm = bpm;
	}
}

void score_sing(int x, int *_name, int *_toname, int srate, int bytes, int channels) {
	string name, toname;
	int ii = 0;
	while (_name[ii]) name.push_back((char)_name[ii++]);
	ii = 0;
	while (_toname[ii]) toname.push_back((char)_toname[ii++]);
	system(("del __" + name + "_000.wav").c_str());
	system(("del __" + name + "_0.wav").c_str());
	system(("del __" + name + "_1.wav").c_str());
	system(("del " + toname).c_str());
	int id = 0, cur = 0;
	vector<pair<int, int>> note_ids; // (bpm, note_id)
	for (int i = 0; i < scores[x].bars.size(); i++) {
		for (int j = 0; j < scores[x].bars[i].notes.size(); j++) {
			note_ids.push_back({scores[x].bars[i].bpm, scores[x].bars[i].notes[j]});
		}
	}
	double cur_dur = 0;
	int lasthalf = 0;
	bool lastrest = 0;
	for (int i = 0; i < note_ids.size(); i++) {
		int bpm = note_ids[i].first;
		int note_id = note_ids[i].second;
		bool needs_out =
		    (i + 1 == note_ids.size() || notes[note_ids[i + 1].second].rest_or_tie != 2);
		cur_dur += notes[note_id].duration_beat * 60 / bpm;
		if (notes[note_id].rest_or_tie == 0) lasthalf = notes[note_id].half;
		if (notes[note_id].rest_or_tie == 1) lastrest = 1;
		if (needs_out) {
			++cur;
			string nw1name = "__" + name + "_000.wav";
			id ^= 1;
			string nw2name = "__" + name + "_" + to_string(id) + ".wav";
			string nw0name = "__" + name + "_" + to_string(id ^ 1) + ".wav";
			stringstream ss_len, ss_pitch;
			ss_len << fixed << setprecision(10) << cur_dur;
			ss_pitch << fixed << setprecision(10) << pow(2, lasthalf / 12.0);
			if (!lastrest) {
				system(("rubberband-r3 --duration " + ss_len.str() + " --frequency " +
				        ss_pitch.str() + " " + name + " " + nw1name)
				           .c_str());
			} else {
				system(("sox -n -r " + to_string(srate) + " -c " + to_string(channels) + " -b " +
				        to_string(bytes) + " " + nw1name + " trim 0 " + ss_len.str())
				           .c_str());
				// 44100 might needs changing
			}
			if (cur > 1) system(("sox " + nw0name + " " + nw1name + " " + nw2name).c_str());
			else system(("copy /y " + nw1name + " " + nw2name).c_str());
			lasthalf = 0;
			cur_dur = 0;
			lastrest = 0;
		}
	}
	// 当前 id 就是最终结果
	string finalname = "__" + name + "_" + to_string(id) + ".wav";
	system(("rename " + finalname + " " + toname).c_str());
}

void putint(int x) { printf("%d", x); }

void putch(int x) { printf("%c", (char)x); }

int getint() {
	int x;
	scanf("%d", &x);
	return x;
}
}