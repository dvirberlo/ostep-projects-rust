#include<stdio.h>
#include<stdlib.h>
#include<string.h>

int main(int argc, char *argv[]);
void grep(char term[], FILE *fp);

int main(int argc, char *argv[]){
    if (argc < 2){
        printf("wgrep: searchterm [file ...]\n");
        return EXIT_FAILURE;
    }
    else if (argc < 3) grep(argv[1], stdin);
    else{
        FILE *fp = fopen(argv[2], "r");
        if (fp == NULL) {
            printf("wgrep: cannot open file\n");
            exit(1);
        }
        grep(argv[1], fp);
        fclose(fp);
    }
    
    return EXIT_SUCCESS;
}

void grep(char term[], FILE *fp){
    char *line  = NULL;
    size_t len  = 0;
    ssize_t read;
    while ((read = getline(&line, &len, fp)) != -1){
        if(strstr(line, term) != NULL) printf("%s", line);
    }
    if (line) free(line);
}
