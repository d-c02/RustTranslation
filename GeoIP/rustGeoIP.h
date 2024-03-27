#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct GeoIPTag {
  char *file_path;
	unsigned char *cache;
	unsigned char *index_cache;
	unsigned int *databaseSegments;
	char databaseType;
	time_t mtime;
	int flags;
	off_t	size;
	char record_length;
	int charset; /* 0 iso-8859-1 1 utf8 */
	int record_iter; /* used in GeoIP_next_record */
	int netmask; /* netmask of last lookup - set using depth in _GeoIP_seek_record */
	time_t last_mtime_check;
        off_t dyn_seg_size; /* currently only used by the cityconfidence database */
        unsigned int ext_flags; /* bit 0 teredo support enabled */
} GeoIP;

uint32_t geoip_addr_to_num(const char *ipstr);

int geoip_enable_teredo(GeoIP* gi, int true_false);
