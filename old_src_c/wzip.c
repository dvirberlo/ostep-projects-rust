#include<stdio.h>
#include<stdlib.h>
#include<math.h>

int main(int argc, char *argv[]);
void zip(int fpc, FILE *fp[]);
char getNext(int fpc, FILE *fp[], int *ifp);
void writeZip(char c, u_int32_t count);

int main(int argc, char *argv[]){
    if (argc < 2){
        printf("wzip: file1 [file2 ...]\n");
        return EXIT_FAILURE;
    }
    else{
        int fpc = argc -1;
        FILE *fp[fpc];
        for (int i = 0; i < fpc; i++){
            fp[i] = fopen(argv[1], "r");
            if (fp == NULL) {
                printf("wzip: cannot open file\n");
                return EXIT_FAILURE;
            }
        }
        zip(fpc, fp);
        for(int i = 0; i < fpc; i++) fclose(fp[i]);
        
    }
    
    return EXIT_SUCCESS;
}

void zip(int fpc, FILE *fp[]){
    int ifp = 0;
    char last = 0;
    char current;
    u_int32_t counter = 0;
    while( (current = getNext(fpc, fp, &ifp)) != EOF ){
        if( (current == last || last == 0) && counter < (int) pow(8, 4)){
            counter++;
        }
        else{
            writeZip(last, counter);
            counter = 1;
        }
        last = current;
    }
    writeZip(last, counter);
}

char getNext(int fpc, FILE *fp[], int *ifp){
    char current;
    if ( (current = (char) fgetc(fp[*ifp])) != EOF ){
        return current;
    }
    else{
        ++*ifp;
        if(*ifp == fpc) return EOF;
        current = (char) fgetc(fp[*ifp]);
        return current;
    }
}

void writeZip(char c, u_int32_t count){
    fwrite(&count, sizeof(u_int32_t), 1, stdout);
    fputc((int)c, stdout);
}
