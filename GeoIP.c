#include <stdio.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <stdint.h>
#include "GeoIP/rustGeoIP.h"


unsigned long GeoIP_addr_to_num(const char *addr)
{
	unsigned int    c, octet, t;
	unsigned long   ipnum;
	int             i = 3;

	octet = ipnum = 0;
	while ((c = *addr++)) {
		if (c == '.') {
			if (octet > 255)
				return 0;
			ipnum <<= 8;
			ipnum += octet;
			i--;
			octet = 0;
		} else {
			t = octet;
			octet <<= 3;
			octet += t;
			octet += t;
			c -= '0';
			if (c > 9)
				return 0;
			octet += c;
		}
	}
	if ((octet > 255) || (i != 0))
		return 0;
	ipnum <<= 8;
	return ipnum + octet;
}

int GeoIP_enable_teredo(GeoIP* gi, int true_false){
  unsigned int mask = ( 1U << 0 );
  int b = ( gi->ext_flags & mask ) ? 1 : 0;
  gi->ext_flags &= ~mask ;
  if ( true_false )
    gi->ext_flags |= true_false;
  return b;
}

int main(int argc, char *argv[])
{
	GeoIP * gi;
	gi = (GeoIP *)malloc(sizeof(GeoIP));
	gi->ext_flags = 1;
	printf("%d\n", geoip_enable_teredo(gi, 1));
	free(gi);
}
