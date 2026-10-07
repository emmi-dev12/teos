/* TeOS home: framebuffer + mouse. Click a box. Keys still work (1-0). */
#include <dirent.h>
#include <fcntl.h>
#include <linux/fb.h>
#include <linux/input.h>
#include <poll.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <unistd.h>

static uint8_t *fb;
static int fbw, fbh, fbline, bpp;
static uint32_t roff, goff, boff;
static int mx, my, down;

static uint32_t px(uint8_t r, uint8_t g, uint8_t b) {
  return ((uint32_t)r << roff) | ((uint32_t)g << goff) | ((uint32_t)b << boff);
}

static void putp(int x, int y, uint32_t c) {
  if ((unsigned)x >= (unsigned)fbw || (unsigned)y >= (unsigned)fbh) return;
  uint8_t *p = fb + y * fbline + x * (bpp / 8);
  if (bpp == 32) *(uint32_t *)p = c;
  else if (bpp == 16) {
    uint16_t v = (uint16_t)(((c >> 3) & 0x1f) | ((c >> 5) & 0x7e0) | ((c >> 8) & 0xf800));
    *(uint16_t *)p = v;
  }
}

static void fill(int x, int y, int w, int h, uint32_t c) {
  for (int j = 0; j < h; j++)
    for (int i = 0; i < w; i++) putp(x + i, y + j, c);
}

static const unsigned char GLYPH[36][7] = {
    /* 0-9 */
    {0x0e, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0e},
    {0x04, 0x0c, 0x04, 0x04, 0x04, 0x04, 0x0e},
    {0x0e, 0x11, 0x01, 0x06, 0x08, 0x10, 0x1f},
    {0x0e, 0x11, 0x01, 0x06, 0x01, 0x11, 0x0e},
    {0x02, 0x06, 0x0a, 0x12, 0x1f, 0x02, 0x02},
    {0x1f, 0x10, 0x1e, 0x01, 0x01, 0x11, 0x0e},
    {0x06, 0x08, 0x10, 0x1e, 0x11, 0x11, 0x0e},
    {0x1f, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08},
    {0x0e, 0x11, 0x11, 0x0e, 0x11, 0x11, 0x0e},
    {0x0e, 0x11, 0x11, 0x0f, 0x01, 0x02, 0x0c},
    /* A-Z */
    {0x0e, 0x11, 0x11, 0x1f, 0x11, 0x11, 0x11},
    {0x1e, 0x11, 0x11, 0x1e, 0x11, 0x11, 0x1e},
    {0x0e, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0e},
    {0x1e, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1e},
    {0x1f, 0x10, 0x10, 0x1e, 0x10, 0x10, 0x1f},
    {0x1f, 0x10, 0x10, 0x1e, 0x10, 0x10, 0x10},
    {0x0e, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0e},
    {0x11, 0x11, 0x11, 0x1f, 0x11, 0x11, 0x11},
    {0x0e, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0e},
    {0x01, 0x01, 0x01, 0x01, 0x11, 0x11, 0x0e},
    {0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11},
    {0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1f},
    {0x11, 0x1b, 0x15, 0x15, 0x11, 0x11, 0x11},
    {0x11, 0x19, 0x15, 0x13, 0x11, 0x11, 0x11},
    {0x0e, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0e},
    {0x1e, 0x11, 0x11, 0x1e, 0x10, 0x10, 0x10},
    {0x0e, 0x11, 0x11, 0x11, 0x15, 0x12, 0x0d},
    {0x1e, 0x11, 0x11, 0x1e, 0x14, 0x12, 0x11},
    {0x0e, 0x11, 0x10, 0x0e, 0x01, 0x11, 0x0e},
    {0x1f, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04},
    {0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0e},
    {0x11, 0x11, 0x11, 0x11, 0x11, 0x0a, 0x04},
    {0x11, 0x11, 0x11, 0x15, 0x15, 0x1b, 0x11},
    {0x11, 0x11, 0x0a, 0x04, 0x0a, 0x11, 0x11},
    {0x11, 0x11, 0x0a, 0x04, 0x04, 0x04, 0x04},
    {0x1f, 0x01, 0x02, 0x04, 0x08, 0x10, 0x1f},
};

static void glyph(int x, int y, char ch, uint32_t c) {
  int gi = -1;
  if (ch >= '0' && ch <= '9') gi = ch - '0';
  else if (ch >= 'A' && ch <= 'Z') gi = 10 + (ch - 'A');
  else if (ch >= 'a' && ch <= 'z') gi = 10 + (ch - 'a');
  if (gi < 0) return;
  for (int row = 0; row < 7; row++) {
    unsigned char bits = GLYPH[gi][row];
    for (int col = 0; col < 5; col++)
      if (bits & (0x10 >> col)) putp(x + col, y + row, c);
  }
}

static void text(int x, int y, const char *s, uint32_t c) {
  while (*s) {
    glyph(x, y, *s, c);
    x += 6;
    s++;
  }
}

typedef struct {
  int x, y, w, h;
  char code;
  const char *label;
} Tile;

static Tile tiles[10];
static int ntiles;

static void layout(void) {
  int pad = 16, gap = 10;
  int cols = 2;
  int tw = (fbw - pad * 2 - gap) / cols;
  int th = 48;
  if (th < 36) th = 36;
  int rows = 5;
  int y0 = 56;
  const char *labs[] = {"NAME", "APPS", "NOTE", "FILE", "WEB", "STAT", "TERM", "CALC", "BOOT", "OFF"};
  const char codes[] = {'1', '2', '3', '4', '5', '6', '7', '8', '9', '0'};
  ntiles = 10;
  for (int i = 0; i < 10; i++) {
    int col = i % cols, row = i / cols;
    tiles[i].x = pad + col * (tw + gap);
    tiles[i].y = y0 + row * (th + gap);
    tiles[i].w = tw;
    tiles[i].h = th;
    tiles[i].code = codes[i];
    tiles[i].label = labs[i];
  }
}

static uint32_t COL_BG, COL_BAR, COL_TILE, COL_INK, COL_HOT, COL_CUR;

static void draw(int hi) {
  fill(0, 0, fbw, fbh, COL_BG);
  fill(0, 0, fbw, 40, COL_BAR);
  text(12, 14, "TEOS", COL_INK);
  text(48, 14, "TAP A BOX", px(180, 220, 255));
  for (int i = 0; i < ntiles; i++) {
    Tile *t = &tiles[i];
    fill(t->x, t->y, t->w, t->h, i == hi ? COL_HOT : COL_TILE);
    int lx = t->x + 12;
    int ly = t->y + (t->h - 7) / 2;
    text(lx, ly, t->label, COL_INK);
  }
  /* cursor */
  fill(mx - 1, my - 6, 3, 13, COL_CUR);
  fill(mx - 6, my - 1, 13, 3, COL_CUR);
}

static char hit(int x, int y) {
  for (int i = 0; i < ntiles; i++) {
    Tile *t = &tiles[i];
    if (x >= t->x && x < t->x + t->w && y >= t->y && y < t->y + t->h) return t->code;
  }
  return 0;
}

static int open_fb(void) {
  int fd = open("/dev/fb0", O_RDWR);
  if (fd < 0) return -1;
  struct fb_var_screeninfo v;
  struct fb_fix_screeninfo f;
  if (ioctl(fd, FBIOGET_VSCREENINFO, &v) || ioctl(fd, FBIOGET_FSCREENINFO, &f)) {
    close(fd);
    return -1;
  }
  fbw = v.xres;
  fbh = v.yres;
  bpp = v.bits_per_pixel;
  fbline = f.line_length;
  roff = v.red.offset;
  goff = v.green.offset;
  boff = v.blue.offset;
  if (bpp < 16) {
    close(fd);
    return -1;
  }
  size_t n = (size_t)f.smem_len;
  fb = mmap(NULL, n, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
  if (fb == MAP_FAILED) {
    close(fd);
    return -1;
  }
  mx = fbw / 2;
  my = fbh / 2;
  return fd;
}

static int evfds[16];
static int is_mice[16];
static int nev;

static void scan_input(void) {
  DIR *d = opendir("/dev/input");
  if (!d) return;
  struct dirent *e;
  while ((e = readdir(d))) {
    if (strncmp(e->d_name, "event", 5) != 0) continue;
    if (nev >= 16) break;
    char path[64];
    snprintf(path, sizeof path, "/dev/input/%s", e->d_name);
    int fd = open(path, O_RDONLY | O_NONBLOCK);
    if (fd >= 0) {
      evfds[nev] = fd;
      is_mice[nev] = 0;
      nev++;
    }
  }
  closedir(d);
  int m = open("/dev/input/mice", O_RDONLY | O_NONBLOCK);
  if (m >= 0 && nev < 16) {
    evfds[nev] = m;
    is_mice[nev] = 1;
    nev++;
  }
}

static int absmax_x = 32767, absmax_y = 32767;

static char handle_mice(int fd) {
  unsigned char b[3];
  ssize_t n = read(fd, b, 3);
  if (n != 3) return 0;
  mx += (signed char)b[1];
  my -= (signed char)b[2];
  if (mx < 0) mx = 0;
  if (my < 0) my = 0;
  if (mx >= fbw) mx = fbw - 1;
  if (my >= fbh) my = fbh - 1;
  if ((b[0] & 1) && !down) {
    down = 1;
    return hit(mx, my);
  }
  if (!(b[0] & 1)) down = 0;
  return 0;
}

static char handle_ev(int fd) {
  struct input_event ev;
  ssize_t n = read(fd, &ev, sizeof ev);
  if (n != sizeof ev) return 0;
  if (ev.type == EV_REL) {
    if (ev.code == REL_X) mx += ev.value;
    if (ev.code == REL_Y) my += ev.value;
    if (mx < 0) mx = 0;
    if (my < 0) my = 0;
    if (mx >= fbw) mx = fbw - 1;
    if (my >= fbh) my = fbh - 1;
  } else if (ev.type == EV_ABS) {
    if (ev.code == ABS_X) mx = (int)((long)ev.value * fbw / (absmax_x ? absmax_x : 32767));
    if (ev.code == ABS_Y) my = (int)((long)ev.value * fbh / (absmax_y ? absmax_y : 32767));
    if (ev.code == ABS_MT_POSITION_X) mx = (int)((long)ev.value * fbw / 32767);
    if (ev.code == ABS_MT_POSITION_Y) my = (int)((long)ev.value * fbh / 32767);
  } else if (ev.type == EV_KEY && (ev.code == BTN_LEFT || ev.code == BTN_TOUCH || ev.code == BTN_MOUSE)) {
    if (ev.value == 1) {
      down = 1;
      return hit(mx, my);
    }
    down = 0;
  }
  return 0;
}

int main(int argc, char **argv) {
  int fbfd = open_fb();
  if (fbfd < 0) {
    fprintf(stderr, "teos-gui: no framebuffer\n");
    return 2;
  }
  COL_BG = px(8, 8, 12);
  COL_BAR = px(20, 80, 160);
  COL_TILE = px(28, 32, 48);
  COL_INK = px(245, 245, 247);
  COL_HOT = px(40, 140, 220);
  COL_CUR = px(255, 255, 255);
  layout();
  if (argc > 1 && strcmp(argv[1], "smoke") == 0) {
    draw(-1);
    printf("fb %dx%d bpp%d\n", fbw, fbh, bpp);
    return 0;
  }
  scan_input();
  int kfd = open("/dev/tty", O_RDONLY | O_NONBLOCK);
  if (kfd < 0) kfd = STDIN_FILENO;
  draw(-1);
  for (;;) {
    struct pollfd p[20];
    int np = 0;
    p[np].fd = kfd;
    p[np].events = POLLIN;
    np++;
    for (int i = 0; i < nev; i++) {
      p[np].fd = evfds[i];
      p[np].events = POLLIN;
      np++;
    }
    poll(p, np, 40);
    char got = 0;
    if (p[0].revents & POLLIN) {
      char ch;
      if (read(kfd, &ch, 1) == 1) {
        if (ch >= '0' && ch <= '9') got = ch;
        if (ch == '\n' || ch == ' ') got = '2'; /* default apps? no, ignore */
        if (ch == 'q') got = '0';
      }
    }
    for (int i = 1; i < np && !got; i++) {
      if (p[i].revents & POLLIN) {
        int idx = i - 1;
        got = is_mice[idx] ? handle_mice(p[i].fd) : handle_ev(p[i].fd);
      }
    }
    draw(-1);
    if (got) {
      printf("%c\n", got);
      fflush(stdout);
      return 0;
    }
  }
}
