#include<stdio.h>
#include<stdlib.h>
#include<math.h>

int main(int argc, char *argv[]);
void unzip(FILE *fp);
u_int32_t readZipCount(FILE *fp);

int main(int argc, char *argv[]){
    if (argc < 2){
        printf("wunzip: file1 [file2 ...]\n");
        return EXIT_FAILURE;
    }
    else{
        for (int i = 1; i < argc; i++){
            FILE *fp = fopen(argv[i], "r");
            if (fp == NULL) {
                printf("wunzip: cannot open file\n");
                return EXIT_FAILURE;
            }
            unzip(fp);
            fclose(fp);
        }
    }
    
    return EXIT_SUCCESS;
}

void unzip(FILE *fp){
    u_int32_t zipCount = 0;
    while ( (zipCount = readZipCount(fp)) != 0 ){
        char c = (char) fgetc(fp);
        for(int i = 0; i < zipCount; i++){
            fputc((int) c, stdout);
        }
    }
}

u_int32_t readZipCount(FILE *fp){
    u_int32_t zipCount = 0;
    fread(&zipCount, sizeof(u_int32_t), 1, fp);
    return zipCount;
}
