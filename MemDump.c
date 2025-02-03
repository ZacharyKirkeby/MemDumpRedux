#include <stdio.h>
#include <string.h>
#include <stdlib.h>

void mymemdump(FILE * fd, char * p , int len) {

    for (int k = 0; k < len/16; k++) {
      // prints memory location, p + iteration * 16 to handle number of iterations
      fprintf(fd, "0x%016lX: ", (unsigned long) p+(k*16));

      // loop for the hex values
      for (int i = 0; i < 16; i++) {
        int c = p[(k*16)+i]&0XFF; // Get value at [p]. The &0xFF is to make sure you truncate to 8bits or one byte.

        // Print first byte as hexadecimal
        fprintf(fd, "%02X ", c);
      }
      fprintf(fd, " ");

      // print the ascii values
      for (int j = 0; j < 16; j++) {
        int c2 = p[(k*16)+j]&0XFF;
        // prints ascii character if its between 32 and 127, else .
        fprintf(fd, "%c", (c2>=32 && c2<=127)?c2:'.');
      }
      fprintf(fd, "\n");
    }

    // overhang case
    if (len % 16) {
      // len/16 * 16 finds comparative position / leftovers
      fprintf(fd, "0x%016lX: ", (unsigned long) p + (len/16 * 16));

      //handles hex
      for (int i2 = (len/16 * 16); i2 < (len/16 * 16 + 16); i2++) {
        if (i2 < len) {
          int c3 = p[i2]&0XFF; // Get value at [p]. The &0xFF is to make sure you truncate to 8bits or one byte.
          fprintf(fd, "%02X ", c3);
        } else {
          // prints spaces to ensure in line
          fprintf(fd, "   ");
        }
      }
      fprintf(fd, " ");
      // prints ascii, same rules as upper loop
      for (int j2 = (len/16 * 16); j2 < len; j2++) {
        int c4 = p[j2]&0XFF;
        fprintf(fd, "%c", (c4>=32 && c4<=127)?c4:'.');
      }
      fprintf(fd, "\n");
    }
}