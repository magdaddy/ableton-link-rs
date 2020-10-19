#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct WLink WLink;
typedef struct WSessionState WSessionState;
typedef struct WClock WClock;
typedef void (*RustClosurePtr)(void*, WSessionState*, WLink*);

// Link

WLink* Link_create(double bpm);
void Link_destroy(WLink* lp);

bool Link_isEnabled(WLink* link);
void Link_enable(WLink* lp, bool enable);

bool Link_isStartStopSyncEnabled(WLink* lp);
void Link_enableStartStopSync(WLink* lp, bool enable);

size_t Link_numPeers(WLink* lp);

void Link_setNumPeersCallback(WLink* lp, void (*callback)(size_t));
void Link_setTempoCallback(WLink* lp, void (*callback)(double));
void Link_setStartStopCallback(WLink* lp, void (*callback)(bool));

WClock* Link_clock(WLink* lp);

void Link_withAudioSessionState(WLink* lp, RustClosurePtr cp, void* closure_data);
void Link_commitAudioSessionState(WLink* lp, WSessionState* ssp);

// WSessionState* Link_captureAppSessionState(WLink* lp);

void Link_withAppSessionState(WLink* lp, RustClosurePtr cp, void* closure_data);
void Link_commitAppSessionState(WLink* lp, WSessionState* ssp);

// SessionState

// void SessionState_destroy(WSessionState* ssp);

double SessionState_tempo(WSessionState* sp);
void SessionState_setTempo(WSessionState* ssp, double bpm, int64_t atTime);

double SessionState_beatAtTime(WSessionState* ssp, int64_t time, double quantum);
double SessionState_phaseAtTime(WSessionState* ssp, int64_t time, double quantum);
int64_t SessionState_timeAtBeat(WSessionState* sp, double beat, double quantum);
void SessionState_requestBeatAtTime(WSessionState* ssp, double beat, int64_t time, double quantum);
void SessionState_forceBeatAtTime(WSessionState* ssp, double beat, int64_t time, double quantum);

void SessionState_setIsPlaying(WSessionState* ssp, bool isPlaying, int64_t time);
bool SessionState_isPlaying(WSessionState* sp);

int64_t SessionState_timeForIsPlaying(WSessionState* ssp);
void SessionState_requestBeatAtStartPlayingTime(WSessionState* ssp, double beat, double quantum);
void SessionState_setIsPlayingAndRequestBeatAtTime(WSessionState* ssp, bool isPlaying, int64_t time, double beat, double quantum);

// Clock

void Clock_destroy(WClock* cp);

#if defined(__APPLE__)
int64_t Clock_ticksToMicros(WClock* cp, uint64_t ticks);
uint64_t Clock_microsToTicks(WClock* cp, int64_t micros);

uint64_t Clock_ticks(WClock* cp);
#endif

int64_t Clock_micros(WClock* cp);


#ifdef __cplusplus
}
#endif
