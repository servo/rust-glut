
typedef void (*fty_glutMainLoopEvent)();
typedef void (*fty_glutInit)(int*, char**);
typedef void (*fty_glutInitDisplayMode)(int);
typedef int (*fty_glutCreateWindow)(char*);
typedef void (*fty_glutDestroyWindow)(int);
typedef void (*fty_glutPostRedisplay)();
typedef void (*fty_glutSwapBuffers)();
typedef int (*fty_glutGetWindow)();
typedef void (*fty_glutSetWindow)(int);
typedef void (*fty_glutReshapeWindow)(int ,int);
typedef void (*fty_glutDisplayFunc)(void*);
typedef void (*fty_glutReshapeFunc)(void*);
typedef void (*fty_glutKeyboardFunc)(void*);
typedef void (*fty_glutMouseFunc)(void*);
typedef void (*fty_glutMotionFunc)(void*);
typedef void (*fty_glutPassiveMotionFunc)(void*);
typedef void (*fty_glutTimerFunc)(unsigned int,void*, int);
typedef int (*fty_glutGet)(int);
typedef int (*fty_glutGetModifiers)();
typedef void (*fty_glutSetWindowTitle)(char*);
typedef void (*fty_glutIdleFunc)(void*);
typedef void (*fty_glutInitWindowSize)(int, int);
typedef void (*fty_glutMouseWheelFunc)(void*);

fty_glutMainLoopEvent fn_glutMainLoopEvent;
fty_glutInit fn_glutInit;
fty_glutInitDisplayMode fn_glutInitDisplayMode;
fty_glutCreateWindow fn_glutCreateWindow;
fty_glutDestroyWindow fn_glutDestroyWindow;
fty_glutPostRedisplay fn_glutPostRedisplay;
fty_glutSwapBuffers fn_glutSwapBuffers;
fty_glutGetWindow fn_glutGetWindow;
fty_glutSetWindow fn_glutSetWindow;
fty_glutReshapeWindow fn_glutReshapeWindow;
fty_glutDisplayFunc fn_glutDisplayFunc;
fty_glutReshapeFunc fn_glutReshapeFunc;
fty_glutKeyboardFunc fn_glutKeyboardFunc;
fty_glutMouseFunc fn_glutMouseFunc;
fty_glutMotionFunc fn_glutMotionFunc;
fty_glutPassiveMotionFunc fn_glutPassiveMotionFunc;
fty_glutTimerFunc fn_glutTimerFunc;
fty_glutGet fn_glutGet;
fty_glutGetModifiers fn_glutGetModifiers;
fty_glutSetWindowTitle fn_glutSetWindowTitle;
fty_glutIdleFunc fn_glutIdleFunc;
fty_glutInitWindowSize fn_glutInitWindowSize;
fty_glutMouseWheelFunc fn_glutMouseWheelFunc;

void reg_fn_glutMainLoopEvent(fty_glutMainLoopEvent fn)
{
    fn_glutMainLoopEvent = fn;
}
void reg_fn_glutInit(fty_glutInit fn)
{
    fn_glutInit = fn;
}
void reg_fn_glutInitDisplayMode(fty_glutInitDisplayMode fn)
{
    fn_glutInitDisplayMode = fn;
}
void reg_fn_glutCreateWindow(fty_glutCreateWindow fn)
{
    fn_glutCreateWindow = fn;
}
void reg_fn_glutDestroyWindow(fty_glutDestroyWindow fn)
{
    fn_glutDestroyWindow = fn;
}
void reg_fn_glutPostRedisplay(fty_glutPostRedisplay fn)
{
    fn_glutPostRedisplay = fn;
}
void reg_fn_glutSwapBuffers(fty_glutSwapBuffers fn)
{
    fn_glutSwapBuffers = fn;
}
void reg_fn_glutGetWindow(fty_glutGetWindow fn)
{
    fn_glutGetWindow = fn;
}
void reg_fn_glutSetWindow(fty_glutSetWindow fn)
{
    fn_glutSetWindow = fn;
}
void reg_fn_glutReshapeWindow(fty_glutReshapeWindow fn)
{
    fn_glutReshapeWindow = fn;
}
void reg_fn_glutDisplayFunc(fty_glutDisplayFunc fn)
{
    fn_glutDisplayFunc = fn;
}
void reg_fn_glutReshapeFunc(fty_glutReshapeFunc fn)
{
    fn_glutReshapeFunc = fn;
}
void reg_fn_glutKeyboardFunc(fty_glutKeyboardFunc fn)
{
    fn_glutKeyboardFunc = fn;
}
void reg_fn_glutMouseFunc(fty_glutMouseFunc fn)
{
    fn_glutMouseFunc = fn;
}
void reg_fn_glutMotionFunc(fty_glutMotionFunc fn)
{
    fn_glutMotionFunc = fn;
}
void reg_fn_glutPassiveMotionFunc(fty_glutPassiveMotionFunc fn)
{
    fn_glutPassiveMotionFunc = fn;
}
void reg_fn_glutTimerFunc(fty_glutTimerFunc fn)
{
    fn_glutTimerFunc = fn;
}
void reg_fn_glutGet(fty_glutGet fn)
{
    fn_glutGet = fn;
}
void reg_fn_glutGetModifiers(fty_glutGetModifiers fn)
{
    fn_glutGetModifiers = fn;
}
void reg_fn_glutSetWindowTitle(fty_glutSetWindowTitle fn)
{
    fn_glutSetWindowTitle = fn;
}
void reg_fn_glutIdleFunc(fty_glutIdleFunc fn)
{
    fn_glutIdleFunc = fn;
}
void reg_fn_glutInitWindowSize(fty_glutInitWindowSize fn)
{
    fn_glutInitWindowSize = fn;
}
void reg_fn_glutMouseWheelFunc(fty_glutMouseWheelFunc fn)
{
    fn_glutMouseWheelFunc = fn;
}

void glutMainLoopEvent()
{
    fn_glutMainLoopEvent();
}
void glutInit(int* argcp, char** argv)
{
    fn_glutInit(argcp, argv);
}
void glutInitDisplayMode(int mode)
{
    fn_glutInitDisplayMode(mode);
}
 int glutCreateWindow(char* title)
{
    return fn_glutCreateWindow(title);
}
void glutDestroyWindow(int win)
{
    fn_glutDestroyWindow(win);
}
void glutPostRedisplay()
{
    fn_glutPostRedisplay();
}
void glutSwapBuffers()
{
    fn_glutSwapBuffers();
}
int glutGetWindow()
{
    return fn_glutGetWindow();
}
void glutSetWindow(int win)
{
    fn_glutSetWindow(win);
}
void glutReshapeWindow(int width, int height)
{
    fn_glutReshapeWindow(width, height);
}
void glutDisplayFunc(void* fn)
{
    fn_glutDisplayFunc(fn);
}
void glutReshapeFunc(void* fn)
{
    fn_glutReshapeFunc(fn);
}
void glutKeyboardFunc(void* fn)
{
    fn_glutKeyboardFunc(fn);
}
void glutMouseFunc(void* fn)
{
    fn_glutMouseFunc(fn);
}
void glutMotionFunc(void* fn)
{
    fn_glutMotionFunc(fn);
}
void glutPassiveMotionFunc(void* fn)
{
    fn_glutPassiveMotionFunc(fn);
}
void glutTimerFunc(unsigned int millis,void* fn, int value)
{
    fn_glutTimerFunc(millis, fn, value);
}
int glutGet(int type)
{
    return fn_glutGet(type);
}
int glutGetModifiers()
{
    return fn_glutGetModifiers();
}
void glutSetWindowTitle(char* title)
{
    fn_glutSetWindowTitle(title);
}
void glutIdleFunc(void* fn)
{
    fn_glutIdleFunc(fn);
}
void glutInitWindowSize(int width, int height)
{
    fn_glutInitWindowSize(width, height);
}
void glutMouseWheelFunc(void* fn)
{
    fn_glutMouseWheelFunc(fn);
}
