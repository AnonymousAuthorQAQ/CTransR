#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define Error( Str )        FatalError( Str )
#define FatalError( Str )   fprintf( stderr, "%s\n", Str ), exit( 1 )

typedef int ElementType;

struct TreeNode;
typedef struct TreeNode *Position;
typedef struct TreeNode *SearchTree;

SearchTree MakeEmpty( SearchTree T );
Position Find( ElementType X, SearchTree T );
Position FindMin( SearchTree T );
Position FindMax( SearchTree T );
SearchTree Insert( ElementType X, SearchTree T );
SearchTree Delete( ElementType X, SearchTree T );
ElementType Retrieve( Position P );
void PrintInOrder( SearchTree T );
SearchTree DeleteEvenNumbers(SearchTree T);

struct TreeNode
{
    ElementType Element;
    SearchTree  Left;
    SearchTree  Right;
};

SearchTree MakeEmpty( SearchTree T )
{
    if( T != NULL )
    {
        MakeEmpty( T->Left );
        MakeEmpty( T->Right );
        free( T );
    }
    return NULL;
}

Position Find( ElementType X, SearchTree T )
{
    if( T == NULL )
        return NULL;
    if( X < T->Element )
        return Find( X, T->Left );
    else if( X > T->Element )
        return Find( X, T->Right );
    else
        return T;
}

Position FindMin( SearchTree T )
{
    if( T == NULL )
        return NULL;
    else if( T->Left == NULL )
        return T;
    else
        return FindMin( T->Left );
}

Position FindMax( SearchTree T )
{
    if( T != NULL )
        while( T->Right != NULL )
            T = T->Right;
    return T;
}

SearchTree Insert( ElementType X, SearchTree T )
{
    if( T == NULL )
    {
        T = malloc( sizeof( struct TreeNode ) );
        if( T == NULL )
            FatalError( "Out of space!!!" );
        else
        {
            T->Element = X;
            T->Left = T->Right = NULL;
        }
    }
    else if( X < T->Element )
        T->Left = Insert( X, T->Left );
    else if( X > T->Element )
        T->Right = Insert( X, T->Right );

    return T;
}

SearchTree Delete( ElementType X, SearchTree T )
{
    Position TmpCell;

    if( T == NULL )
        Error( "Element not found" );
    else if( X < T->Element )  
        T->Left = Delete( X, T->Left );
    else if( X > T->Element )  
        T->Right = Delete( X, T->Right );
    else if( T->Left && T->Right )  
    {
        TmpCell = FindMin( T->Right );
        T->Element = TmpCell->Element;
        T->Right = Delete( T->Element, T->Right );
    }
    else  
    {
        TmpCell = T;
        if( T->Left == NULL ) 
            T = T->Right;
        else if( T->Right == NULL )
            T = T->Left;
        free( TmpCell );
    }

    return T;
}

ElementType Retrieve( Position P )
{
    return P->Element;
}

void PrintInOrder( SearchTree T )
{
    if( T != NULL )
    {
        PrintInOrder( T->Left );
        printf( "%d\n", T->Element );
        PrintInOrder( T->Right );
    }
}

SearchTree DeleteEvenNumbers(SearchTree T)
{
    if (T == NULL)
        return NULL;

    T->Left = DeleteEvenNumbers(T->Left);
    T->Right = DeleteEvenNumbers(T->Right);

    if (T->Element % 2 == 0)
    {
        T = Delete(T->Element, T);
    }

    return T;
}

int main()
{
    SearchTree T;
    Position P;
    char *line = NULL;
    size_t len = 0;
    ssize_t read;
    char *token;
    int num;
    int hasInput = 0;

    T = MakeEmpty(NULL);

    if ((read = getline(&line, &len, stdin)) != -1)
    {
        token = strtok(line, " ");
        while (token != NULL)
        {
            // Skip empty tokens and check if the token contains digits
            if (strlen(token) > 0 && strspn(token, "0123456789") == strlen(token))
            {
                num = atoi(token);
                T = Insert(num, T);
                hasInput = 1;  // Indicate that we have at least one valid input
            }
            token = strtok(NULL, " ");
        }
    }

    free(line);

    if (!hasInput || T == NULL) {
        printf("Tree is empty\n");
        return 0;
    }

    printf("Tree after insertion:\n");
    PrintInOrder(T);

    // Delete all even numbers
    T = DeleteEvenNumbers(T);

    printf("Tree after deleting even numbers:\n");
    PrintInOrder(T);

    if (T != NULL) {
        printf("Min is %d, Max is %d\n", Retrieve(FindMin(T)), Retrieve(FindMax(T)));
    } else {
        printf("Tree is empty\n");
    }

    T = MakeEmpty(T);

    return 0;
}
