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
	int bpm;
	Bar() {
		notes.clear();
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

void bar_setbpm(int x, int bpm) { bars[x].bpm = bpm; }

void bar_push(int x, int note) {
	bars[x].notes.push_back(note);
	cout << "add note with length " << notes[note].duration_beat << endl;
}

void bar_inc_pitch(int x, int semitones) {
	for (int i = 0; i < bars[x].notes.size(); i++) {
		int note_id = bars[x].notes[i];
		Note tmp = notes[note_id];
		if (tmp.rest_or_tie == 0) {
			notes.push_back({tmp.rest_or_tie, tmp.half + semitones, tmp.duration_beat});
			bars[x].notes[i] = notes.size() - 1;
		}
	}
}

void bar_set_duration(int x, int len_ms) {
	double sum = 0;
	for (int i = 0; i < bars[x].notes.size(); i++) {
		sum += notes[bars[x].notes[i]].duration_beat;
	}
	if (sum == 0) return;
	// sum * 1000 / len_ms * 60 拍，60 秒
	bar_setbpm(x, sum * 60000 / len_ms);
}

void bar_inc_pitch_2(Bar &x, int semitones) {
	for (int i = 0; i < x.notes.size(); i++) {
		int note_id = x.notes[i];
		Note tmp = notes[note_id];
		if (tmp.rest_or_tie == 0) {
			notes.push_back({tmp.rest_or_tie, tmp.half + semitones, tmp.duration_beat});
			x.notes[i] = notes.size() - 1;
		}
	}
}

struct Score {
	vector<Bar> bars;
	int bpm;
	Score() {
		bars.clear();
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

void score_setbpm(int x, int bpm) {
	for (int i = 0; i < scores[x].bars.size(); i++) {
		scores[x].bars[i].bpm = bpm;
	}
}

void score_inc_pitch(int x, int semitones) {
	for (int i = 0; i < scores[x].bars.size(); i++) {
		bar_inc_pitch_2(scores[x].bars[i], semitones);
	}
}

void score_set_duration(int x, int len_ms) {
	double sum = 0;
	for (int i = 0; i < scores[x].bars.size(); i++) {
		for (int j = 0; j < scores[x].bars[i].notes.size(); j++) {
			sum += notes[scores[x].bars[i].notes[j]].duration_beat;
		}
	}
	if (sum == 0) return;
	score_setbpm(x, sum * 60000 / len_ms);
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

struct Track {
    string name;
};

vector<Track> tracks;

int newtrack() {
    int id = tracks.size();
    tracks.push_back({""});
    return id;
}

void track_load(int x, int *_name) {
    string name;
    int ii = 0;
    while (_name[ii]) name.push_back((char)_name[ii++]);
    tracks[x].name = name;
} // load 就是直接放进去

void track_copy(int x, int *_name1, int *_name2) { // name1 copy to (x, name2)
    string name1, name2;
    int ii = 0;
    while (_name1[ii]) name1.push_back((char)_name1[ii++]);
    ii = 0;
    while (_name2[ii]) name2.push_back((char)_name2[ii++]);
    system(("copy /y " + name1 + " " + name2).c_str());
    tracks[x].name = name2;
}

void track_append(int x, int y) { // x += y
    string name1 = tracks[x].name;
    string name2 = tracks[y].name;
	system(("del __tmp_" + name1 + ".wav").c_str());
    system(("sox " + name1 + " " + name2 + " __tmp_" + name1+ ".wav").c_str());
    system(("copy /y __tmp_" + name1 + ".wav " + name1).c_str());
}

void track_append_slience(int x, int len_ms, int srate, int bytes, int channels) {
    string name = tracks[x].name;
    system(("del __tmp_" + name + ".wav").c_str());
    stringstream ss_len;
    ss_len << fixed << setprecision(10) << len_ms / 1000.0;
    system(("sox -n -r " + to_string(srate) + " -c " + to_string(channels) + " -b " +
            to_string(bytes) + " __tmp_" + name + ".wav trim 0 " + ss_len.str())
               .c_str());
    system(("copy /y __tmp_" + name + ".wav " + name).c_str());
}

void track_set_volume(int x, int fz, int fm) {
    string name = tracks[x].name;
    stringstream ss_vol;
    ss_vol << fixed << setprecision(10) << 1.0 * fz / fm;
    system(("sox " + name + " " + name + " vol " + ss_vol).c_str());
}

void track_stack(int x, int y) { // x += y
    string name1 = tracks[x].name;
    string name2 = tracks[y].name;
    system(("del __tmp_" + name1 + ".wav").c_str());
    system(("sox -M " + name1 + " " + name2 + " __tmp_" + name1 + ".wav").c_str());
    system(("copy /y __tmp_" + name1 + ".wav " + name1).c_str());
}

// struct Audio {
//     string name;
// };

// vector<Audio> audios;

// int newaudio() {
//     int id = audios.size();
//     audios.push_back({""});
//     return id;
// }

void putint(int x) { printf("%d", x); }

void putch(int x) { printf("%c", (char)x); }

int getint() {
	int x;
	scanf("%d", &x);
	return x;
}
}