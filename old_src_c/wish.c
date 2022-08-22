#include<stdio.h>
#include<stdlib.h>
#include <string.h>
#include <unistd.h>

int main(int argc, char* argv[]);
void wishLine(char* line);

void Werror();
char** Wargs(char* line);

void Wexit(char* line);
void Wcd(char* line);
void Wpath(char* line);

char error_message[30] = "An error has occurred\n\0";

int main(int argc, char* argv[]){
    if(argc < 2){
        // loop mode
        char* line = NULL;
        size_t len  = 0;
        while(1){
            printf("wish> ");
            getline(&line, &len, stdin);
            wishLine(line);
        }
    }
    else{
        FILE* fp = fopen(argv[1], "r");
        if (fp == NULL) {
            printf("wish: cannot open file\n");
            return EXIT_FAILURE;
        }
        char* line = NULL;
        size_t len  = 0;
        while (getline(&line, &len, fp) != -1) wishLine(line);
        fclose(fp);
        return EXIT_SUCCESS;
    }
}

void wishLine(char* line){
    // replace last char (= LF) with space
    line[strlen(line)-1] = ' ';
    char* firstC = strsep(&line, " ");
    if (strcmp(firstC, "") == 0) return;
    else if (strcmp(firstC, "exit") == 0) Wexit(line);
    else if (strcmp(firstC, "cd") == 0) Wcd(line);
    else if (strcmp(firstC, "path") == 0) Wpath(line);
    else Werror();
}

char** Wargs(char* line){
    // TODO
    return NULL;
}
void Werror(){
    printf("%s", error_message);
}

// wish commands:
void Wexit(char* line){
    exit(EXIT_SUCCESS);
}

void Wcd(char* line){
    // check atleast 1 args and no more
    char* path = strsep(&line, " ");
    if (strcmp(path, "") == 0 || strcmp(line, "") != 0){
        Werror();
        return;
    }
    if (chdir(path) != EXIT_SUCCESS) Werror();
}

void Wpath(char* line){
    char* path;
    while ((path = strsep(&line, " ")) != NULL){
        access(path, X_OK);
        // TODO: what's going on here?
    }
}
